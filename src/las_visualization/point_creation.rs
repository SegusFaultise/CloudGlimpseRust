use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

use crate::las_file_handler::las_file_parser::Point3D;


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
    let offset_x = min_x + range_x / 2.0;
    let offset_y = min_y + range_y / 2.0;
    let offset_z = min_z + range_z / 2.0;
    let max_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    let min_color: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    let scale_factor = 1.5 / range_x.max(range_y).max(range_z);
    let positions: Vec<[f32; 3]> = points
        .iter()
        .map(|point| [
            (point.x as f32 - offset_x) * scale_factor,
            (point.z as f32 - offset_z) * scale_factor,
            (point.y as f32 - offset_y) * scale_factor,
        ])
        .collect();
    let colors: Vec<[f32; 4]> = points
        .iter()
        .map(|point| {
            let t = (point.z as f32 - min_height) / (max_height - min_height);
            let r = min_color[0] + t * (max_color[0] - min_color[0]);
            let g = min_color[1] + t * (max_color[1] - min_color[1]);
            let b = min_color[2] + t * (max_color[2] - min_color[2]);
            let a = min_color[3] + t * (max_color[3] - min_color[3]);
            //let color = height_to_color(point.z as f32, min_height, max_height);
            [r, g, b, a]
        })
        .collect();
    let mut mesh = Mesh::new(PrimitiveTopology::PointList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    return mesh;
}