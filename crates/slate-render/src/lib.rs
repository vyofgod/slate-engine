//! # Slate Render
//!
//! Headless wgpu renderer. Consumes a slice of [`AtomicInstruction`],
//! batches the [`RenderPrimitive::FillRect`] (and stroke) variants
//! into **one instanced draw call**, and writes the result into an
//! offscreen RGBA8 texture.
//!
//! There is no surface / window here — that's the integration layer's
//! job. Phase 2 renders to a texture that can be read back, dumped to
//! a PPM for the demo, or blitted to whatever swapchain the embedder
//! provides.
//!
//! ## Batching
//!
//! Every `FillRect` / `StrokeRect` in the AIS stream becomes one
//! instance. A single vertex buffer holds the 6 corners of a unit
//! quad; a single instance buffer holds `(origin, size, color)` per
//! rect. The draw call count is `1`, regardless of how many rects
//! you submit.
//!
//! ## Scheduling
//!
//! [`FrameScheduler`] keeps the engine's rendering cadence separated
//! from wall time. Pass it a target Hz (e.g. 300 for a 300-Hz panel)
//! and it hands out deterministic `frame_index` values the kernel
//! can fold into its state-transition pipeline.

pub mod frame;

use bytemuck::{Pod, Zeroable};
use slate_ais::{AtomicInstruction, Rect, RenderPrimitive, Rgba8};
use std::sync::Arc;

pub use frame::FrameScheduler;

// ---------- GPU-facing types ------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Uniforms {
    viewport: [f32; 2],
    _pad:     [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Corner {
    pos: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Instance {
    origin: [f32; 2],
    size:   [f32; 2],
    color:  [f32; 4],
}

const QUAD: [Corner; 6] = [
    Corner { pos: [0.0, 0.0] },
    Corner { pos: [1.0, 0.0] },
    Corner { pos: [0.0, 1.0] },
    Corner { pos: [0.0, 1.0] },
    Corner { pos: [1.0, 0.0] },
    Corner { pos: [1.0, 1.0] },
];

const MAX_INSTANCES: u64 = 4096;

/// How the renderer was configured.
#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
    pub width:  u32,
    pub height: u32,
}

impl Default for RenderConfig {
    fn default() -> Self { RenderConfig { width: 256, height: 256 } }
}

/// Boxed error from wgpu or the renderer itself.
#[derive(Debug)]
pub enum RenderError {
    NoAdapter,
    Device(wgpu::RequestDeviceError),
    ReadBack,
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::NoAdapter => write!(f, "no compatible wgpu adapter"),
            RenderError::Device(e) => write!(f, "device init failed: {e}"),
            RenderError::ReadBack  => write!(f, "pixel readback failed"),
        }
    }
}

impl std::error::Error for RenderError {}

// ---------- The renderer ----------------------------------------------------

pub struct Renderer {
    device:   Arc<wgpu::Device>,
    queue:    Arc<wgpu::Queue>,
    cfg:      RenderConfig,
    target:   wgpu::Texture,
    view:     wgpu::TextureView,
    readback: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    vbuf:     wgpu::Buffer,
    ibuf:     wgpu::Buffer,
    // Held to keep the uniform buffer alive for the bind group.
    #[allow(dead_code)]
    ubuf:     wgpu::Buffer,
    bind:     wgpu::BindGroup,
}

impl Renderer {
    /// Initialize a headless wgpu device and build the pipeline.
    /// Blocks the current thread; intended to be called once at
    /// engine start.
    pub fn new(cfg: RenderConfig) -> Result<Self, RenderError> {
        pollster::block_on(Self::new_async(cfg))
    }

    pub async fn new_async(cfg: RenderConfig) -> Result<Self, RenderError> {
        let instance = wgpu::Instance::default();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::HighPerformance,
                compatible_surface:     None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or(RenderError::NoAdapter)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label:             Some("slate.device"),
                    required_features: wgpu::Features::empty(),
                    required_limits:   wgpu::Limits::downlevel_defaults(),
                    memory_hints:      wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .map_err(RenderError::Device)?;

        let device = Arc::new(device);
        let queue  = Arc::new(queue);

        // Offscreen target.
        let target = device.create_texture(&wgpu::TextureDescriptor {
            label:         Some("slate.target"),
            size:          wgpu::Extent3d {
                width:                 cfg.width,
                height:                cfg.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count:    1,
            dimension:       wgpu::TextureDimension::D2,
            format:          wgpu::TextureFormat::Rgba8UnormSrgb,
            usage:           wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats:    &[],
        });
        let view = target.create_view(&wgpu::TextureViewDescriptor::default());

        // Pixel readback buffer — rows padded to 256-byte alignment.
        let bytes_per_row = pad_256(cfg.width * 4);
        let readback = device.create_buffer(&wgpu::BufferDescriptor {
            label:              Some("slate.readback"),
            size:               (bytes_per_row * cfg.height) as u64,
            usage:              wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // Static quad + dynamic instance buffer.
        let vbuf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("slate.quad"),
            size:  std::mem::size_of_val(&QUAD) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&vbuf, 0, bytemuck::cast_slice(&QUAD));

        let ibuf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("slate.instances"),
            size:  MAX_INSTANCES * std::mem::size_of::<Instance>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let ubuf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("slate.uniforms"),
            size:  std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(
            &ubuf,
            0,
            bytemuck::bytes_of(&Uniforms {
                viewport: [cfg.width as f32, cfg.height as f32],
                _pad:     [0.0, 0.0],
            }),
        );

        // Bind group: one uniform, visible to the vertex stage.
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label:   Some("slate.bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding:    0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty:         wgpu::BindingType::Buffer {
                    ty:                 wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size:   None,
                },
                count: None,
            }],
        });
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label:   Some("slate.bg"),
            layout:  &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding:  0,
                resource: ubuf.as_entire_binding(),
            }],
        });

        // Shader + pipeline.
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label:  Some("slate.rect"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/rect.wgsl").into()),
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("slate.layout"),
            bind_group_layouts:   &[&bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("slate.pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module:              &shader,
                entry_point:         Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers:             &[
                    // vertex corners
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Corner>() as u64,
                        step_mode:    wgpu::VertexStepMode::Vertex,
                        attributes:   &[wgpu::VertexAttribute {
                            format:          wgpu::VertexFormat::Float32x2,
                            offset:          0,
                            shader_location: 0,
                        }],
                    },
                    // per-instance origin/size/color
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Instance>() as u64,
                        step_mode:    wgpu::VertexStepMode::Instance,
                        attributes:   &[
                            wgpu::VertexAttribute {
                                format:          wgpu::VertexFormat::Float32x2,
                                offset:          0,
                                shader_location: 1,
                            },
                            wgpu::VertexAttribute {
                                format:          wgpu::VertexFormat::Float32x2,
                                offset:          8,
                                shader_location: 2,
                            },
                            wgpu::VertexAttribute {
                                format:          wgpu::VertexFormat::Float32x4,
                                offset:          16,
                                shader_location: 3,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module:              &shader,
                entry_point:         Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets:             &[Some(wgpu::ColorTargetState {
                    format:     wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive:     wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample:   wgpu::MultisampleState::default(),
            multiview:     None,
            cache:         None,
        });

        Ok(Self {
            device,
            queue,
            cfg,
            target,
            view,
            readback,
            pipeline,
            vbuf,
            ibuf,
            ubuf,
            bind,
        })
    }

    /// Render one frame from the given AIS stream.
    ///
    /// All non-render primitives are skipped silently — they are not
    /// this subsystem's concern. The render stream is expected to be
    /// already in document order; no sorting happens here.
    pub fn render(&mut self, stream: &[AtomicInstruction]) {
        let mut instances: Vec<Instance> = Vec::new();
        for instr in stream {
            if let AtomicInstruction::Render(p) = instr {
                collect(p, &mut instances);
            }
        }

        let count = instances.len().min(MAX_INSTANCES as usize) as u32;
        if count > 0 {
            self.queue.write_buffer(&self.ibuf, 0, bytemuck::cast_slice(&instances[..count as usize]));
        }

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("slate.frame"),
        });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("slate.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view:           &self.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set:      None,
                timestamp_writes:         None,
            });

            if count > 0 {
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &self.bind, &[]);
                pass.set_vertex_buffer(0, self.vbuf.slice(..));
                pass.set_vertex_buffer(1, self.ibuf.slice(..));
                pass.draw(0..6, 0..count);
            }
        }

        self.queue.submit([encoder.finish()]);
    }

    /// Copy the target texture into the readback buffer and map it.
    /// Returns tightly-packed RGBA8 bytes (unpadded rows).
    pub fn read_pixels(&self) -> Result<Vec<u8>, RenderError> {
        pollster::block_on(self.read_pixels_async())
    }

    pub async fn read_pixels_async(&self) -> Result<Vec<u8>, RenderError> {
        let bytes_per_row_padded = pad_256(self.cfg.width * 4);

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("slate.readback.encoder"),
        });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture:   &self.target,
                mip_level: 0,
                origin:    wgpu::Origin3d::ZERO,
                aspect:    wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &self.readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset:         0,
                    bytes_per_row:  Some(bytes_per_row_padded),
                    rows_per_image: Some(self.cfg.height),
                },
            },
            wgpu::Extent3d {
                width:                 self.cfg.width,
                height:                self.cfg.height,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit([encoder.finish()]);

        let (tx, rx) = futures_channel::<Result<(), wgpu::BufferAsyncError>>();
        let slice = self.readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, move |r| { let _ = tx(r); });
        self.device.poll(wgpu::Maintain::Wait);
        rx.await.map_err(|_| RenderError::ReadBack)?.map_err(|_| RenderError::ReadBack)?;

        let data = slice.get_mapped_range();

        // Strip row padding into a tightly-packed output.
        let w = self.cfg.width as usize;
        let h = self.cfg.height as usize;
        let padded = bytes_per_row_padded as usize;
        let mut out = Vec::with_capacity(w * h * 4);
        for y in 0..h {
            let row = &data[y * padded..y * padded + w * 4];
            out.extend_from_slice(row);
        }
        drop(data);
        self.readback.unmap();
        Ok(out)
    }

    pub fn config(&self) -> RenderConfig { self.cfg }
}

// ---------- helpers ---------------------------------------------------------

fn collect(p: &RenderPrimitive, out: &mut Vec<Instance>) {
    match p {
        RenderPrimitive::FillRect { rect, color } => out.push(inst(*rect, *color)),
        RenderPrimitive::StrokeRect { rect, color, width } => {
            // Cheap stroke: 4 sub-rects (top, bottom, left, right).
            let w  = *width;
            let r  = *rect;
            let c  = *color;
            let o  = r.origin;
            let sz = r.size;
            // top
            out.push(inst(
                Rect::from_ltwh(o.x.raw(), o.y.raw(), sz.w.raw(), w),
                c,
            ));
            // bottom
            out.push(inst(
                Rect::from_ltwh(o.x.raw(), o.y.raw() + sz.h.raw() - w, sz.w.raw(), w),
                c,
            ));
            // left
            out.push(inst(
                Rect::from_ltwh(o.x.raw(), o.y.raw(), w, sz.h.raw()),
                c,
            ));
            // right
            out.push(inst(
                Rect::from_ltwh(o.x.raw() + sz.w.raw() - w, o.y.raw(), w, sz.h.raw()),
                c,
            ));
        }
        // Phase 2: text/path/layer primitives still TODO. Skip cleanly.
        _ => {}
    }
}

fn inst(r: Rect, c: Rgba8) -> Instance {
    Instance {
        origin: [r.origin.x.raw(), r.origin.y.raw()],
        size:   [r.size.w.raw(),   r.size.h.raw()],
        color:  [
            c.r as f32 / 255.0,
            c.g as f32 / 255.0,
            c.b as f32 / 255.0,
            c.a as f32 / 255.0,
        ],
    }
}

const fn pad_256(n: u32) -> u32 {
    let rem = n % 256;
    if rem == 0 { n } else { n + (256 - rem) }
}

// Minimal oneshot channel — dep-free. futures-channel isn't in this
// crate and pulling another crate for one await would be overkill.
fn futures_channel<T>() -> (
    impl FnOnce(T) -> Result<(), T>,
    impl std::future::Future<Output = Result<T, ()>>,
) {
    use std::sync::{Arc, Mutex};
    use std::task::Waker;
    struct State<T> { value: Option<T>, waker: Option<Waker>, closed: bool }
    let s = Arc::new(Mutex::new(State::<T> { value: None, waker: None, closed: false }));
    let st = s.clone();
    let send = move |v: T| -> Result<(), T> {
        let mut g = st.lock().unwrap();
        if g.closed { return Err(v); }
        g.value = Some(v);
        if let Some(w) = g.waker.take() { w.wake(); }
        Ok(())
    };
    let recv = async move {
        std::future::poll_fn(move |cx| {
            let mut g = s.lock().unwrap();
            if let Some(v) = g.value.take() {
                std::task::Poll::Ready(Ok(v))
            } else {
                g.waker = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
        })
        .await
    };
    (send, recv)
}

// ---------- ppm helper for the demo -----------------------------------------

/// Write RGBA8 pixels to a P6 PPM file (ignores alpha). No PNG
/// dependency, no image crate — trivially readable by any viewer.
pub fn write_ppm(path: &std::path::Path, rgba: &[u8], w: u32, h: u32) -> std::io::Result<()> {
    use std::io::Write;
    let mut f = std::fs::File::create(path)?;
    writeln!(f, "P6\n{w} {h}\n255")?;
    let mut row = Vec::with_capacity(w as usize * 3);
    for y in 0..h {
        row.clear();
        for x in 0..w {
            let i = ((y * w + x) * 4) as usize;
            row.extend_from_slice(&rgba[i..i + 3]);
        }
        f.write_all(&row)?;
    }
    Ok(())
}
