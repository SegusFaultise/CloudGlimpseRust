use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy_panorbit_camera::PanOrbitCameraPlugin;

use wasm_bindgen::prelude::*;

mod las_file_handler;
use las_file_handler::las_file_parser::read_las_file;

mod las_visualization;
use las_visualization::bevy_setup::{setup, LasFileData};

/// A material for rendering points, containing color information.
///
/// This struct is used to define how points are rendered, including their color.
/// It binds a color to a uniform in the shader.
#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct PointMaterial {

    /// The color of the points. This is bound to the shader as a uniform.
    #[uniform(0)]
    color: Color,
}

/// Implementation of the `Material` trait for `PointMaterial`.
///
/// This provides the necessary shader reference for rendering.
impl Material for PointMaterial {

    /// Specifies the fragment shader to use for rendering points.
    ///
    /// # Returns
    /// Returns a reference to the shader file.
    fn fragment_shader() -> ShaderRef {
        "shaders/PointMaterial.wgsl".into()
    }
}

/// The entry point for the WebAssembly application.
///
/// This function is called when the WebAssembly module is executed.
/// It reads LAS file data, initializes the application, and runs it.
///
/// # Arguments
/// * `file_data` - A byte slice reference to the contents of a LAS file.
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
