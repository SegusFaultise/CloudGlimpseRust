use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

use crate::las_file_handler::las_file_parser::Point3D;

/// Converts a given height value to a color.
///
/// The color is determined based on the normalized height value,
/// interpolated between the minimum and maximum heights.
///
/// # Arguments
/// * `height` - The height value to convert.
/// * `min_height` - The minimum height in the range.
/// * `max_height` - The maximum height in the range.
///
/// # Returns
/// Returns a `Color` value representing the height.
pub fn height_to_color(height: f32, min_height: f32, max_height: f32) -> Color {
    let normalized_height = (height - min_height) / (max_height - min_height);
    
    let r = normalized_height;
    let g = 0.5;
    let b = 1.0 - normalized_height;

    return Color::rgb(r, g, b);
}

/// Creates a mesh from a vector of `Point3D`.
///
/// Each point is converted to a position in the mesh, and a color is calculated
/// based on the z-value of the point.
///
/// # Arguments
/// * `points` - A reference to a vector of `Point3D` from which the mesh will be created.
/// * `min_height` - The minimum height value used for normalizing point height for color calculation.
/// * `max_height` - The maximum height value used for normalizing point height for color calculation.
///
/// # Returns
/// Returns a `Mesh` representing the points.
pub fn create_point_mesh_from_point3d(points: &Vec<Point3D>, min_height: f32, max_height: f32) -> Mesh {
    let (min_x, max_x) = points.iter().fold((f32::MAX, f32::MIN), |(min, max), p| (min.min(p.x as f32), max.max(p.x as f32)));
    let (min_y, max_y) = points.iter().fold((f32::MAX, f32::MIN), |(min, max), p| (min.min(p.y as f32), max.max(p.y as f32)));
    let (min_z, max_z) = points.iter().fold((f32::MAX, f32::MIN), |(min, max), p| (min.min(p.z as f32), max.max(p.z as f32)));

    let range_x = max_x - min_x;
    let range_y = max_y - min_y;
    let range_z = max_z - min_z;

    let d_value = 2.0;

    let offset_x = min_x + range_x / d_value;
    let offset_y = min_y + range_y / d_value;
    let offset_z = min_z + range_z / d_value;

    let scale_factor = 1.5 / range_x.max(range_y).max(range_z);

    let positions: Vec<[f32; 3]> = points
        .iter()
        .map(|point| [
            (point.x as f32 - offset_x) * scale_factor,
            (point.y as f32 - offset_y) * scale_factor,
            (point.z as f32 - offset_z) * scale_factor,
        ])
        .collect();

    let colors: Vec<[f32; 4]> = points
        .iter()
        .map(|point| {
            let color = height_to_color(point.z as f32, min_height, max_height);
            [color.r(), color.g(), color.b(), 1.0]
        })
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::PointList);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    return mesh;
}