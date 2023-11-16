#import bevy_pbr::forward_io::VertexOutput
@group(1) @binding(0) var<uniform> material: PointMaterial;


struct PointMaterial {
    color: vec4<f32>,
};




@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material.color;
}