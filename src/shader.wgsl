// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    // let x = f32(1 - i32(in_vertex_index)) * 0.5;
    // let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    // // The same as
    // // vec2<f32>(0.0, 0.5),
    // // vec2<f32>(-0.5, -0.5),
    // // vec2<f32>(0.5, -0.5)
    // out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    var pos = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5, -0.5),
        vec2<f32>(0.5, -0.5)
    );

    out.clip_position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
    out.position = pos[in_vertex_index];
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
    // return vec4<f32>(in.position, 0.5, 1.0);
}