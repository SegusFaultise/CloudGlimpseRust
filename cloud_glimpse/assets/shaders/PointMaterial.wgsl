//#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}


@group(1) @binding(0) var<uniform> max_color: vec4<f32>;
@group(1) @binding(1) var<uniform> min_color: vec4<f32>;
@group(1) @binding(2) var<uniform> max_height: f32;
@group(1) @binding(3) var<uniform> min_height: f32;
struct Vertex {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position;
    // Calculate scale based on Z position
    

    var out: VertexOutput;
    // NOTE: Passing 0 as the instance_index to get_model_matrix() is a hack
    // for this example as the instance_index builtin would map to the wrong
    // index in the Mesh array. This index could be passed in via another
    // uniform instead but it's unnecessary for the example.
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(0u),
        vec4<f32>(position, 1.0)
    );
    let scale = (position.z - min_height) / (max_height - min_height);
    // Interpolate color based on scale
    out.color = mix(min_color, max_color, scale);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}