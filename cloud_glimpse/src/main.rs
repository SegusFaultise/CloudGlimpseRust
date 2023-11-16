use bevy::ecs::query;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
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
use std::f32::consts::TAU;
use std::path::Path;
use las_file_handler::las_file_parser::{read_las_file_header, read_las_file, read_point_record, print_las_header_info,Point3D};

mod las_file_handler;

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct PointMaterial {
    #[uniform(0)]
    max_color: Color,
    #[uniform(1)]
    min_color: Color,
    #[uniform(2)]
    max_height: f32,
    #[uniform(3)]
    min_height: f32,
}

impl Material for PointMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/PointMaterial.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/PointMaterial.wgsl".into()
    }

}


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_plugins(DefaultPlugins)
    .add_plugins(LogDiagnosticsPlugin::default())
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
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
    let mut point_records = match read_las_file(file_path) {
        Ok(it) => it,
        Err(err) => return println!("Failed to load las file!"),
    };
    // find the average point
    let mut average_x = 0.0;
    let mut average_y = 0.0;
    let mut average_z = 0.0;
    // find the max and min height
    let mut max_height = f32::MIN;
    let mut min_height = f32::MAX;
    let count = point_records.len() as f64;
    for point in &point_records {
        if (point.z as f32) > max_height {
            max_height = point.z as f32;
        }
        if (point.z as f32) < min_height {
            min_height = point.z as f32;
        }
        average_x += point.x/count;
        average_y += point.y/count;
        average_z += point.z/count;
    }
     // apply the offset to all the points
     for point in &mut point_records {
        point.x -= average_x;
        point.y -= average_y;
        point.z -= average_z;
    }
    max_height -= average_z as f32;
    min_height -= average_z as f32; 
    println!("Max Height: {}", max_height);
    println!("Min Height: {}", min_height);
    println!("Average X: {}", average_x);
    println!("Average Y: {}", average_y);
    println!("Average Z: {}", average_z);
   
    // Spawn in the points
    let points_mesh_handle: Handle<Mesh> = meshes.add(create_point_mesh_from_Point3D(&point_records));
    commands.spawn(MaterialMeshBundle {
        
        // offset the points so that the average point is at the origin
        //transform: Transform::from_xyz((-1.0 * average_x) as f32, (-1.0 * average_y) as f32, (-1.0 * average_z) as f32),
        // rotate the points around the x axis such that the z and y axis swap
        transform: Transform::from_rotation(Quat::from_rotation_x((-90.0_f32).to_radians())),
        mesh: points_mesh_handle,
        material: materials.add(PointMaterial {
            max_color: Color::rgb(1.0, 0.0, 0.0),// color for the highest point 
            min_color: Color::rgb(0.0, 0.0, 1.0),// color for the lowest point
            max_height: max_height, // highest point
            min_height: min_height, // lowest point
            }),
            ..Default::default()
    });

    // spawn in the orbit camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, max_height-average_z as f32)),
            ..default()
        },
        PanOrbitCamera{
            zoom_lower_limit: Some(1.0),
            beta:  Some((45.0_f32).to_radians()),// rotate the camera up 45 degrees to start with
            ..Default::default()
        
        },
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
