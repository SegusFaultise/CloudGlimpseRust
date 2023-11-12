#import bevy_pbr::forward_io::VertexOutput

struct MinMaxHeight {
    min_height: f32,
    max_height: f32,
};

@group(1) @binding(0) var<uniform> min_max_height: MinMaxHeight;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let red_value = clamp(mesh.position.y / 100.0, 0.0, 1.0);

    let normalized_height = (mesh.position.y - min_max_height.min_height) / 
                            (min_max_height.max_height - min_max_height.min_height);
    let clamped_height = clamp(normalized_height, 0.0, 1.0);

    return vec4<f32>(clamped_height, 0.0, 0.0, 1.0); // Uncomment this for actual height-based coloring
}

