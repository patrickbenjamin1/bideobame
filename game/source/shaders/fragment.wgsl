// @see ./vertex.wgsl

struct VertexOutput {
    @builtin(position) clip_position : vec4<f32>,
    @location(0) color : vec3<f32>,
};

@fragment
fn fs_main(in : VertexOutput) -> @location(0) vec4<f32> {
    let adjusted_color = in.color * in.clip_position.x * 0.01;

    return vec4<f32>(adjusted_color, 1.0);
}
