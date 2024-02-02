use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, PrimitiveTopology, ShaderRef};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_panorbit_camera::PanOrbitCamera;
use std::path::Path;
use las_file_handler::las_file_parser::{read_las_file_from_file, Point3D};
mod las_file_handler;
use std::sync::Mutex;
use std::env;
use std::f32::consts::TAU;
static GLOBAL_COUNT: Mutex<Count> = Mutex::new(Count { total_points: 0.0 });

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct PointMaterial {
    #[uniform(0)]
    color: Color,
}

#[derive(Debug, Clone, PartialEq, Resource)]
pub struct MinMaxHeightUniform {
    pub min_height: f32,
    pub max_height: f32,
}

impl Material for PointMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/PointMaterial.wgsl".into()
    }
}

fn main() {
    
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn print_point_height_info(average_z: f64, min_height: f32, 
                                     max_height: f32) {
    info!("Height: {}", average_z);
    info!("Min Height: {}", min_height);
    info!("Max Height: {}", max_height);
}

struct Count {
    total_points: f64
}

impl Count {
    // Constructor method to create a new instance of Count
    pub fn new() -> Self {
        Self { total_points: 0.0 }
    }

    // Method to add a value to total_points
    pub fn add(&mut self, value: f64) {
        self.total_points = value;
    }

    // Getter method to retrieve the value of total_points
    pub fn get_total_points(&self) -> f64 {
        self.total_points
    }
}

pub fn get_global_total_points() -> f64 {
    let count = GLOBAL_COUNT.lock().unwrap();
    count.get_total_points()
}

pub fn update_global_count(value: f64) {
    let mut count = GLOBAL_COUNT.lock().unwrap();
    count.add(value);
}

pub fn total_points() {
    let total = get_global_total_points();
    println!("{}", total);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,){
        info!("start");
    let file_path = Path::new("points.las");
    let point_records = match read_las_file_from_file(file_path) {
        Ok(it) => it,
        Err(err) => return warn!("Failed to load las file! | Error: {}", err),
    };

    let mut _average_x: f64 = 0.0;
    let mut _average_y: f64 = 0.0;
    let mut average_z: f64 = 0.0;
    let count = point_records.len() as f64;
    
    update_global_count(count);
    total_points();

    for point in &point_records {
        _average_x += point.x/count;
        _average_y += point.y/count;
        average_z += point.z/count;
    }
    
    let mut min_height = f32::MAX;
    let mut max_height = f32::MIN;
    for point in &point_records {
        let z = point.z as f32;
        min_height = min_height.min(z);
        max_height = max_height.max(z);
    }

    info!("loaded");

    print_point_height_info(average_z, min_height, max_height);
    
    commands.insert_resource(MinMaxHeightUniform {
        min_height,
        max_height,
    });

    let points_mesh_handle: Handle<Mesh> = 
        meshes.add(create_point_mesh_from_point3d(&point_records, 
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
        PanOrbitCamera::default()
    ));

}


fn create_point_mesh_from_point3d(points: &Vec<Point3D>, min_height: f32, max_height: f32) -> Mesh {
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
    let min_color: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
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
