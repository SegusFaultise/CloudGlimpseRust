#import bevy_pbr::forward_io::VertexOutput

struct PointMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0) var<uniform> material: PointMaterial;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material.color;
}