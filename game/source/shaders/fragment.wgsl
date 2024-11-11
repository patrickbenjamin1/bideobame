// @see ./vertex.wgsl

struct VertexOutput {
    @builtin(position) clip_position : vec4<f32>,
    @location(0) color : vec3<f32>,
};

@fragment
fn fs_main(in : VertexOutput) -> @location(0) vec4<f32> {
    // Return color directly without any adjustments
    return vec4<f32>(in.color, 1.0);
}
