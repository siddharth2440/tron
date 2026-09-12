struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>
};

@group(0) @binding(0)
var my_texture: texture_2d<f32>;

@group(0) @binding(1)
var my_sampler: sampler;


struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>
}

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;



@vertex
fn vs_main(
    input: VertexInput
) -> VertexOutput {
    var output: VertexOutput;
    
    output.clip_position = uniforms.projection * uniforms.view * uniforms.model * vec4<f32>( input.position, 1.0 );
    output.tex_coords = input.tex_coords;
    return output;

}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample( my_texture, my_sampler, in.tex_coords );
}
