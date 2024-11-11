struct VertexInput {
    @location(0) position : vec3<f32>,
    @location(1) color : vec3<f32>,
    @location(2) should_wave : u32,
};

struct VertexOutput {
    @builtin(position) clip_position : vec4<f32>,
    @location(0) color : vec3<f32>,
};

fn make_wave(model: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(
        model.x + sin(model.y * 10.0) * 0.1,
        model.y + sin(model.x * 10.0) * 0.1,
        model.z
    );
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // Pass color directly without modification
    out.color = model.color;
    
    var final_position = model.position;
    if (model.should_wave == 1u) {
        final_position = make_wave(model.position);
    }
    
    out.clip_position = vec4<f32>(final_position, 1.0);
    return out;
}