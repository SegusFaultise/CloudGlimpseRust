/*use cloud_glimpse::run;

  fn main() {
  pollster::block_on(run());
  }*/
use bevy::prelude::*;
use bevy::render::color;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::VertexAttributeValues,
        render_resource::{
            AsBindGroup, PrimitiveTopology, ShaderRef,
        },
    },
};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_panorbit_camera::PanOrbitCamera;
use std::path::Path;
use las_file_handler::las_file_parser::{read_las_file_header, read_las_file, read_point_record, print_las_header_info,Point3D};

mod las_file_handler;

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

unsafe impl Send for MinMaxHeightUniform {}
unsafe impl Sync for MinMaxHeightUniform {}

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
        .add_plugins((MaterialPlugin::<PointMaterial>::default()))
        .add_systems(Startup, setup)
        .run();}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    ){
    //let file_path = Path::new("/home/neuralnuts/Downloads/bigp.las");
    let file_path = Path::new("/home/neuralnuts/Downloads/points.las");
    //let file_path = Path::new("points.las");
    let point_records = match read_las_file(file_path) {
        Ok(it) => it,
        Err(err) => return println!("Failed to load las file!"),
    };

    let mut average_x = 0.0;
    let mut average_y = 0.0;
    let mut average_z = 0.0;
    let count = point_records.len() as f64;
    for point in &point_records {
        average_x += point.x/count;
        average_y += point.y/count;
        average_z += point.z/count;
    }
    
    let mut min_height = f32::MAX;
    let mut max_height = f32::MIN;
    for point in &point_records {
        let z = point.z as f32;
        min_height = min_height.min(z);
        max_height = max_height.max(z);
    }

    println!("loaded");
    let color = height_to_color(average_z as f32, min_height as f32, max_height as f32);

    println!("Height: {}", average_z);
    println!("Min Height: {}", min_height);
    println!("Max Height: {}", max_height);
    println!("Calculated Color: {:?}", color);

    commands.insert_resource(MinMaxHeightUniform {
        min_height,
        max_height,
    });

    let points_mesh_handle: Handle<Mesh> = 
        meshes.add(create_point_mesh_from_point3D(&point_records, 
                                                  min_height, 
                                                  max_height));
    commands.spawn(PbrBundle {
        transform: Transform::from_xyz((-1.0 * average_x) as f32, (-1.0 * average_y) as f32, (-1.0 * average_z) as f32),
        mesh: points_mesh_handle,
        ..Default::default()
    });


           //
    commands.spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
                ..default()
            },
            PanOrbitCamera::default(),
            ));

}

/// this is the OG func
fn height_to_color(height: f32, min_height: f32, max_height: f32) -> Color {
    let normalized_height = (height - min_height) / (max_height - min_height);

    let r = normalized_height;        // Red component increases with height
    let g = 0.5;                      // No green component
    let b = 1.0 - normalized_height;  // Blue component decreases with height

    Color::rgb(r, g, b)
}

fn lerp_color(start: Color, end: Color, t: f32) -> Color {
    let start_r = start.r();
    let start_g = start.g();
    let start_b = start.b();
    let end_r = end.r();
    let end_g = end.g();
    let end_b = end.b();

    // Interpolate the red, green, and blue channels separately
    let r = start_r + (end_r - start_r) * t;
    let g = start_g + (end_g - start_g) * t;
    let b = start_b + (end_b - start_b) * t;

    // Create a new color from the interpolated channels
    Color::rgb(r, g, b)
}

fn height_to_color_test(height: f32, min_height: f32, max_height: f32) -> Color {
    // Normalize the height value between 0.0 and 1.0
    let normalized_height = (height - min_height) / (max_height - min_height);

    // Adjust the following thresholds to fine-tune sensitivity
    let thresholds = [
        (0.0, Color::rgb(0.0, 0.0, 0.5)), // Deep Blue
        (0.1, Color::rgb(0.0, 0.0, 1.0)), // Blue
        (0.2, Color::rgb(0.0, 1.0, 1.0)), // Cyan
        (0.3, Color::rgb(0.0, 1.0, 0.0)), // Green
        (0.5, Color::rgb(1.0, 1.0, 0.0)), // Yellow
        (0.7, Color::rgb(1.0, 0.5, 0.0)), // Orange
        (1.0, Color::rgb(1.0, 0.0, 0.0)), // Red
    ];

    // Find the two thresholds that the normalized height falls between
    let (lower, upper) = find_thresholds(normalized_height, &thresholds);

    // Calculate the local normalized height between the lower and upper threshold
    let local_normalized_height = (normalized_height - lower.0) / (upper.0 - lower.0);

    // Interpolate between the two colors
    interpolate_colors(lower.1, upper.1, local_normalized_height)
}

fn find_thresholds<'a>(
    normalized_height: f32,
    thresholds: &'a [(f32, Color)],
) -> (&'a (f32, Color), &'a (f32, Color)) {
    let mut lower = &thresholds[0];
    let mut upper = &thresholds[1];

    for window in thresholds.windows(2) {
        if normalized_height >= window[0].0 && normalized_height <= window[1].0 {
            lower = &window[0];
            upper = &window[1];
            break;
        }
    }

    (lower, upper)
}

fn interpolate_colors(start: Color, end: Color, t: f32) -> Color {
    let r = start.r() + (end.r() - start.r()) * t;
    let g = start.g() + (end.g() - start.g()) * t;
    let b = start.b() + (end.b() - start.b()) * t;

    Color::rgb(r, g, b)
}

fn create_point_mesh_from_Point3DD(points: &Vec<Point3D>) -> Mesh {
    // Create a new mesh using a point list topology, where each vertex is a point.
    Mesh::new(PrimitiveTopology::PointList)
        // Assign a position (Vec3) to each vertex.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            points.iter().map(|point| [point.x as f32, point.y as f32, point.z as f32]).collect::<Vec<[f32; 3]>>(),
            )
}

fn create_point_mesh_from_point3D(points: &Vec<Point3D>, min_height: f32, max_height: f32) -> Mesh {
    let scale_factor = 0.01;

    let positions: Vec<[f32; 3]> = points
        .iter()
        .map(|point| [
            (point.x as f32 - 1756960.0) * scale_factor,
            (point.y as f32 - 5447760.0) * scale_factor,
            point.z as f32 * scale_factor,
        ])
        .collect();

    let colors: Vec<[f32; 4]> = points
        .iter()
        .map(|point| {
            let color = height_to_color(point.z as f32, min_height, max_height);
            [color.r(), color.g(), color.b(), 2.0] // RGBA format
        })
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::PointList);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    return mesh;
}
