//! Phase 2 end-to-end demo.
//!
//!     JS source
//!        │  slate.createElement / slate.setStyle
//!        ▼
//!     slate-script  (Boa + host bridge)
//!        │  Vec<OwnedWebCall>
//!        ▼
//!     slate-kernel  (Dispatcher → State + AIS stream)
//!        │  &[AtomicInstruction]
//!        ▼
//!     slate-render  (wgpu headless, instanced draw)
//!        │  RGBA8 bytes
//!        ▼
//!     out/frame.ppm
//!
//! Run:  `cargo run --bin slate-pipeline --release`

use std::path::PathBuf;

use slate_ais::AtomicInstruction;
use slate_kernel::Kernel;
use slate_render::{write_ppm, RenderConfig, Renderer};
use slate_script::ScriptRuntime;

const SCRIPT: &str = r#"
    // Two rectangles, composed via the `slate` host surface.
    const root = slate.createElement('div');
    slate.setStyle(root, 'width:220;height:140;background:#1a1a2e');

    const box1 = slate.createElement('div');
    slate.appendChild(root, box1, 0);
    slate.setStyle(box1, 'width:160;height:60;background:red');

    const box2 = slate.createElement('div');
    slate.appendChild(root, box2, 1);
    slate.setStyle(box2, 'width:80;height:80;background:#3ad1ff');
"#;

fn main() {
    println!("== Slate Phase 2 pipeline ==");

    // 1. Run JS.
    let mut script = ScriptRuntime::new().expect("script init");
    script.eval(SCRIPT).expect("script eval");
    let calls = script.drain();
    println!("script emitted {} WebCalls", calls.len());

    // 2. Kernel decomposes into AIS and updates state.
    let mut kernel = Kernel::new();
    let stream = kernel
        .submit_batch(calls.iter())
        .expect("kernel batch submit");
    println!(
        "AIS stream:  {} instructions ({} render, {} layout, {} state)",
        stream.len(),
        stream.iter().filter(|i| matches!(i, AtomicInstruction::Render(_))).count(),
        stream.iter().filter(|i| matches!(i, AtomicInstruction::Layout(_))).count(),
        stream.iter().filter(|i| matches!(i, AtomicInstruction::State(_))).count(),
    );

    let snap = kernel.snapshot();
    println!(
        "state:       version={}  nodes={}  root={:?}",
        snap.version, snap.node_count, snap.root
    );

    // 3. Render to a 256×256 offscreen target. Skip gracefully on
    //    systems without a Vulkan/Metal/DX adapter (CI boxes, etc.).
    let cfg = RenderConfig { width: 256, height: 256 };
    let mut renderer = match Renderer::new(cfg) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("no GPU adapter available ({e}); skipping render");
            return;
        }
    };
    renderer.render(&stream);
    let pixels = renderer.read_pixels().expect("readback");

    // 4. Dump to PPM (no PNG dep, universally viewable).
    let out = PathBuf::from("out/frame.ppm");
    std::fs::create_dir_all("out").ok();
    write_ppm(&out, &pixels, cfg.width, cfg.height).expect("write ppm");
    println!("wrote {}", out.display());
}
