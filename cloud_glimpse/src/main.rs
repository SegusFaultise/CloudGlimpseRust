/*use cloud_glimpse::run;

fn main() {
    pollster::block_on(run());
}*/
use bevy::prelude::*;
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
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointMaterial>>,
){
    let file_path = Path::new("points.las");
    let point_records = match read_las_file(file_path) {
        Ok(it) => it,
        Err(err) => return println!("Failed to load las file!"),
    };
    // find the average point
    let mut average_x = 0.0;
    let mut average_y = 0.0;
    let mut average_z = 0.0;
    let count = point_records.len() as f64;
    for point in &point_records {
        average_x += point.x/count;
        average_y += point.y/count;
        average_z += point.z/count;
    }

    println!("loaded");
    // Spawn in the points
    let points_mesh_handle: Handle<Mesh> = meshes.add(create_point_mesh_from_Point3D(&point_records));
    commands.spawn(MaterialMeshBundle {
        // offset the points so that the average point is at the origin
        transform: Transform::from_xyz((-1.0 * average_x) as f32, (-1.0 * average_y) as f32, (-1.0 * average_z) as f32),
        mesh: points_mesh_handle,
        material: materials.add(PointMaterial {
             color: Color::rgb(1.0, 0.0, 0.0) 
            }),
            ..Default::default()
    });
    // spawn in the orbit camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
    
}

fn create_point_mesh_from_Point3D(points: &Vec<Point3D>) -> Mesh {
    // Create a new mesh using a point list topology, where each vertex is a point.
    Mesh::new(PrimitiveTopology::PointList)
        // Assign a position (Vec3) to each vertex.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            points.iter().map(|point| [point.x as f32, point.y as f32, point.z as f32]).collect::<Vec<[f32; 3]>>(),
        )
}
