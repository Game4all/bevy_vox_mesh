use bevy::{prelude::*, core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping}};
use bevy_vox_mesh::VoxMeshPlugin;
use std::f32::consts::PI;

fn main() {
    App::default()
    .add_plugins(DefaultPlugins)
    .add_plugins(VoxMeshPlugin::default())
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stdmats: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(1.0, 1.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: Tonemapping::AcesFitted,
            ..Default::default()
        },
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
    ));
    
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { subdivisions: 2,  size: 5.0 })),
    //     material: stdmats.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..Default::default()
    // });
    
    commands.spawn(PbrBundle {
        transform: Transform::from_scale((0.05, 0.05, 0.05).into())
        * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI))
        * Transform::from_translation(Vec3::new(0., 20., 0.)),
        mesh: assets.load("monu1.vox"),
        material: assets.load("monu1.vox#material"),
        ..Default::default()
    });
}
