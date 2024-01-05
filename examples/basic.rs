use bevy::{prelude::*, core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, prepass::DepthPrepass, core_3d::ScreenSpaceTransmissionQuality, experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasBundle}}};
use bevy_vox_mesh::{VoxMeshPlugin, VoxelSceneBundle};
use std::f32::consts::PI;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

fn main() {
    let mut app = App::new();
    
    app.add_plugins((
        DefaultPlugins,
        PanOrbitCameraPlugin,
        VoxMeshPlugin::default()
    ))
    .add_systems(Startup, setup);
    
    // *Note:* TAA is not _required_ for specular transmission, but
    // it _greatly enhances_ the look of the resulting blur effects.
    // Sadly, it's not available under WebGL.
    #[cfg(not(all(feature = "webgl2", target_arch = "wasm32")))]
    app.insert_resource(Msaa::Off)
    .add_plugins(TemporalAntiAliasPlugin);
    
    app.run();
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
            camera_3d: Camera3d {
                screen_space_specular_transmission_quality: ScreenSpaceTransmissionQuality::High,
                screen_space_specular_transmission_steps: 2,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 1.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: Tonemapping::AcesFitted,
            ..Default::default()
        },
        PanOrbitCamera::default(),
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
        TemporalAntiAliasBundle::default(),
    ));
    
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(Vec3::new(-1.0, -3.0, 0.5), Vec3::Y),
        ..default()
    });
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });
    
    // commands.spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Plane { subdivisions: 2,  size: 5.0 })),
    //         material: stdmats.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //         ..Default::default()
    //     });

        commands.spawn(VoxelSceneBundle {
            scene: assets.load("shapes.vox#Scene"),
            transform: Transform::from_scale(Vec3::splat(0.05)),
        });
        // commands.spawn(SpatialBundle {
        //     transform: Transform::from_scale((0.05, 0.05, 0.05).into())
        //     * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI))
        //     * Transform::from_translation(Vec3::new(0., 20., 0.)),
        //     ..default()
        // }).add_child(scene);
        // commands.spawn(PbrBundle {
        //     transform: Transform::from_scale((0.05, 0.05, 0.05).into())
        //     * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI))
        //     * Transform::from_translation(Vec3::new(0., 20., 0.)),
        //     mesh: assets.load("shapes.vox#cone"),
        //     material: assets.load("shapes.vox#material"),
        //     ..Default::default()
        // });
        // commands.spawn(PbrBundle {
        //         transform: Transform::from_scale((0.05, 0.05, 0.05).into())
        //         * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI))
        //         * Transform::from_translation(Vec3::new(0., 20., 0.)),
        //         mesh: assets.load("monu1.vox"),
        //         material: assets.load("monu1.vox#material"),
        //         ..Default::default()
        //     });
        }
        