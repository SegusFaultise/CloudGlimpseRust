#import bevy_pbr::forward_io::VertexOutput

struct MinMaxHeight {
    min_height: f32,
    max_height: f32,
};

@group(1) @binding(0) var<uniform> min_max_height: MinMaxHeight;

fn gaussian(height: f32, min_height: f32, max_height: f32, standard_deviation: f32) -> f32 {
    let mean = (max_height + min_height) * 0.5;
    let scale = 1.0 / (standard_deviation * sqrt(2.0 * 3.14159265359));
    let exponent = -((height - mean) * (height - mean)) / (2.0 * standard_deviation * standard_deviation);
    return scale * exp(exponent);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // Calculate normalized height and apply the Gaussian function
    let normalized_height = (mesh.position.y - min_max_height.min_height) / 
                            (min_max_height.max_height - min_max_height.min_height);
    let standard_deviation = 0.1 * (min_max_height.max_height - min_max_height.min_height);
    let gaussian_value = gaussian(mesh.position.y, min_max_height.min_height, min_max_height.max_height, standard_deviation);

    // Apply Gaussian value to the red channel as a demonstration of the effect
    let red_value = clamp(gaussian_value, 0.0, 1.0);

    // Return color based on Gaussian-modulated red channel
    return vec4<f32>(red_value, 0.0, 0.0, 1.0); // Red channel now represents Gaussian splattering effect
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) uv: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(position, 1.0);
}