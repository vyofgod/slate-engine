// Instanced solid-color rects.
//
// Per-vertex: a unit quad corner.
// Per-instance: a rect in pixel space + an RGBA color.
// Uniforms: the viewport size, so we can map pixel space → NDC on GPU.

struct Uniforms {
    viewport: vec2<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertIn {
    @location(0) corner: vec2<f32>,        // (0,0)..(1,1)
    @location(1) inst_origin: vec2<f32>,   // px
    @location(2) inst_size:   vec2<f32>,   // px
    @location(3) inst_color:  vec4<f32>,   // rgba (0..1)
};

struct VertOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(v: VertIn) -> VertOut {
    let px = v.inst_origin + v.corner * v.inst_size;
    // px → clip space: [0,w]→[-1,1], y flipped
    let ndc = vec2<f32>(
        (px.x / uniforms.viewport.x) * 2.0 - 1.0,
        1.0 - (px.y / uniforms.viewport.y) * 2.0,
    );
    var out: VertOut;
    out.pos = vec4<f32>(ndc, 0.0, 1.0);
    out.color = v.inst_color;
    return out;
}

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
    return in.color;
}
