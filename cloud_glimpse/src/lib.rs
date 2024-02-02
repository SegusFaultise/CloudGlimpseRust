use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
//use bevy_wasm_window_resize::WindowResizePlugin;
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

pub struct WindowResizePlugin;

impl Plugin for WindowResizePlugin {
    #[cfg(target_arch = "wasm32")]
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_browser_resize);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, _app: &mut App) {}
}

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(
    mut primary_query: bevy::ecs::system::Query<
        &mut bevy::window::Window,
        bevy::ecs::query::With<bevy::window::PrimaryWindow>,
    >,
) {
    for mut window in &mut primary_query {
        let wasm_window = web_sys::window().unwrap();
        let window_width = wasm_window.inner_width().unwrap().as_f64().unwrap() as f32;
        let window_height = wasm_window.inner_height().unwrap().as_f64().unwrap() as f32;
        
        let space_to_add: f32 = 150.0;

        let target_width = window_width - (2.0 * space_to_add);
        let target_height = window_height;
        
        if window.resolution.width() != target_width || window.resolution.height() != target_height {
            window.resolution.set(target_width, target_height);
        }
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
            warn!("Failed to load las file data! | Error: {}", err);
            return;
        },
    };
    info!("TOTAL: {}", point_records.len());

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(LasFileData::new(point_records))
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(WindowResizePlugin)
        .add_systems(Startup, setup)
        .run();
}
