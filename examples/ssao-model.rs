use bevy::{prelude::*, core_pipeline::{bloom::BloomSettings, experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasBundle}}, pbr::ScreenSpaceAmbientOcclusionBundle};
use bevy_vox_scene::VoxScenePlugin;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

fn main() {
    let mut app = App::new();
    
    app.add_plugins((
        DefaultPlugins,
        PanOrbitCameraPlugin,
        VoxScenePlugin,
    ))
    .insert_resource(AmbientLight {
        color: Color::rgb_u8(128, 126, 124),
        brightness: 0.5, 
    })
    .add_systems(Startup, setup);
    
    // *Note:* TAA is not _required_ for SSAO, but
    // it enhances the look of the resulting blur effects.
    // Sadly, it's not available under WebGL.
    #[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
    app.insert_resource(Msaa::Off)
    .add_plugins(TemporalAntiAliasPlugin);
    
    app.run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            transform: Transform::from_xyz(20.0, 10.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera::default(),
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
        #[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
        TemporalAntiAliasBundle::default(),
        EnvironmentMapLight { 
            diffuse_map: assets.load("pisa_diffuse.ktx2"), 
            specular_map: assets.load("pisa_specular.ktx2"),
        },
    )).insert(ScreenSpaceAmbientOcclusionBundle::default());
    
    commands.spawn(PbrBundle {
        // Load a single model using the name assigned to it in MagicaVoxel
        mesh: assets.load("study.vox#computer"),
        // This model has no glass voxels, so we can use the opaque material
        material: assets.load("study.vox#material_opaque"),
        ..Default::default()
    });
}
