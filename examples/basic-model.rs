use bevy::prelude::*;
use bevy_vox_scene::VoxScenePlugin;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins,
        VoxScenePlugin,
    ))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(30.0, 30.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        EnvironmentMapLight { 
            diffuse_map: assets.load("pisa_diffuse.ktx2"), 
            specular_map: assets.load("pisa_specular.ktx2"),
        },
    ));
    
    commands.spawn(PbrBundle {
        // Load a single model using the name assigned to it in MagicaVoxel
        mesh: assets.load("study.vox#desk"),
        // This model has no glass voxels, so we can use the opaque material
        material: assets.load("study.vox#material_opaque"),
        ..Default::default()
    });
}
