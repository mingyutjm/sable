struct ConstantBuffer {
    world_view_proj: mat4x4f,
};
@group(0) @binding(0)
var<uniform> cbuffer: ConstantBuffer;

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec3f,
}

@vertex
fn VS(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = cbuffer.world_view_proj * vec4<f32>(model.position, 1.0);
    out.color = model.color;
    return out;
}

@fragment
fn PS(in: VertexOutput) -> @location(0) vec4f {
    return vec4<f32>(in.color, 1.0);
}