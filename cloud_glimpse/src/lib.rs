use bevy::prelude::*;
use bevy::prelude::{Resource, Window};
use bevy::window::WindowPlugin;
use bevy::render::render_resource::{AsBindGroup, PrimitiveTopology, ShaderRef};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_panorbit_camera::PanOrbitCamera;

use wasm_bindgen::prelude::*;

use std::path::Path;

mod las_file_handler;
use las_file_handler::las_file_parser::{read_las_file, Point3D, get_total_points};

mod las_visualization;
use las_visualization::point_creation::{create_point_mesh_from_point3d, height_to_color};
use las_visualization::bevy_setup::{setup, LasFileData};

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct PointMaterial {
    #[uniform(0)]
    color: Color,
}

impl Material for PointMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/PointMaterial.wgsl".into()
    }
}

#[wasm_bindgen]
pub fn main(file_data: &[u8]) { 
    let point_records = match read_las_file(file_data) {
        Ok(points) => points,
        Err(err) => {
            eprintln!("Failed to load las file data! | Error: {}", err);
            return;
        },
    };
    println!("TOTAL: {}", point_records.len());

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(LasFileData::new(point_records))
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}