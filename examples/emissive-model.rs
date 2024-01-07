use bevy::{prelude::*, core_pipeline::bloom::BloomSettings};
use bevy_vox_scene::VoxScenePlugin;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        PanOrbitCameraPlugin,
        VoxScenePlugin,
    ))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // An hdr and bloom-enabled camera is needed to create the emissive glowing effect
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(-20.0, 10.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera::default(),
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
        EnvironmentMapLight { 
            diffuse_map: assets.load("pisa_diffuse.ktx2"), 
            specular_map: assets.load("pisa_specular.ktx2"),
        },
    ));
    
    commands.spawn(PbrBundle {
        // Load a single model using the name assigned to it in MagicaVoxel
        mesh: assets.load("study.vox#computer"),
        // This model has no glass voxels, so we can use the opaque material
        material: assets.load("study.vox#material_opaque"),
        ..Default::default()
    });
}
