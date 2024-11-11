struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) should_wave: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) should_wave: u32,
};

struct Uniforms {
    time: vec4<f32>,
    projection: mat4x4<f32>,
    view: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Apply wave effect if should_wave is 1
    var position = model.position;
    if (model.should_wave == 1u) {
        position.y += sin(uniforms.time[0] * 4.0 + position.x * 2.0) * 0.2;
    }
    

    out.clip_position = uniforms.projection * uniforms.view * vec4<f32>(position, 1.0);
    
    out.color = model.color;
    out.should_wave = model.should_wave;
    
    return out;
}