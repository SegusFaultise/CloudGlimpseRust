use bevy::prelude::*;
use bevy::prelude::{Resource, Window};
use bevy::window::WindowPlugin;
use bevy::render::render_resource::{AsBindGroup, PrimitiveTopology, ShaderRef};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::las_file_handler::las_file_parser::Point3D;
use crate::las_visualization::point_creation::{height_to_color, create_point_mesh_from_point3d};

/// A struct that holds the minimum and maximum height values.
#[derive(Debug, Clone, PartialEq, Resource)]
pub struct MinMaxHeightUniform {
    pub min_height: f32,
    pub max_height: f32,
}

/// A resource that contains the loaded point data from a LAS file.
#[derive(Resource)]
pub struct LasFileData {

    /// A vector of `Point3D` representing the points from the LAS file.
    points: Vec<Point3D>,
}

impl LasFileData {

    /// Creates a new `LasFileData` instance.
    ///
    /// # Arguments
    /// * `points` - A vector of `Point3D` to be contained in `LasFileData`.
    pub fn new(points: Vec<Point3D>) -> Self {
        Self { points }
    }
}

/// Prints the color and height information of a point.
///
/// # Arguments
/// * `average_z` - The average z value (height) of all points.
/// * `min_height` - The minimum height value.
/// * `max_height` - The maximum height value.
/// * `color` - The color data of the point.
fn print_point_color_and_height_info(average_z: f64, min_height: f32, 
                                     max_height: f32, color: Color) {
    println!("Height: {}", average_z);
    println!("Min Height: {}", min_height);
    println!("Max Height: {}", max_height);
    println!("Calculated Color: {:?}", color);
}

/// Sets up the rendering of points from a LAS file.
///
/// # Arguments
/// * `commands` - Commands for spawning entities and resources.
/// * `mut meshes` - A mutable resource for storing mesh assets.
/// * `las_file_data` - The loaded LAS file data resource.
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    las_file_data: Res<LasFileData>){
   
    let mut _average_x: f64 = 0.0;
    let mut _average_y: f64 = 0.0;
    let mut average_z: f64 = 0.0;
    let count = las_file_data.points.len() as f64;

    for point in &las_file_data.points {
        _average_x += point.x/count;
        _average_y += point.y/count;
        average_z += point.z/count;
    }
    
    let mut min_height = f32::MAX;
    let mut max_height = f32::MIN;
    for point in &las_file_data.points {
        let z = point.z as f32;
        min_height = min_height.min(z);
        max_height = max_height.max(z);
    }

    println!("loaded");
    let color = height_to_color(average_z as f32, min_height as f32, max_height as f32);

    print_point_color_and_height_info(average_z, min_height, max_height, color);
    
    commands.insert_resource(MinMaxHeightUniform {
        min_height,
        max_height,
    });

    let points_mesh_handle: Handle<Mesh> = 
        meshes.add(create_point_mesh_from_point3d(&las_file_data.points, 
                                                  min_height, 
                                                  max_height));
    commands.spawn(PbrBundle {
        mesh: points_mesh_handle,
        ..Default::default()
    });

    commands.spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
                ..default()
            },
            PanOrbitCamera::default(),
    ));
}