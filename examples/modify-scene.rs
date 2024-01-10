use bevy::{prelude::*, core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, core_3d::ScreenSpaceTransmissionQuality, experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasBundle}}, input::keyboard::KeyboardInput};
use bevy_vox_scene::{VoxScenePlugin, VoxelSceneBundle, VoxelNodeReady};
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

/// Uses the [`bevy_vox_scene::VoxelNodeReady`] event to add extra components into the scene graph.
/// Press any key to toggle the computer screen on and off
fn main() {
    let mut app = App::new();
    
    app.add_plugins((
        DefaultPlugins,
        PanOrbitCameraPlugin,
        VoxScenePlugin,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, (
        add_computer_component.run_if(on_event::<VoxelNodeReady>()),
        toggle_computer_state.run_if(on_event::<KeyboardInput>()),
    ));
    
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
                screen_space_specular_transmission_steps: 1,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 1.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: Tonemapping::SomewhatBoringDisplayTransform,
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
    ));
    
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(Vec3::new(1.0, -2.5, 0.85), Vec3::Y),
        ..default()
    });
    
    commands.spawn(VoxelSceneBundle {
        scene: assets.load("study.vox"),
        transform: Transform::from_scale(Vec3::splat(0.05)),
        ..default()
    });
}

#[derive(Component)]
struct Computer {
    is_on: bool,
    on_material: Handle<StandardMaterial>,
    off_material: Handle<StandardMaterial>,
}

impl Computer {
    fn material(&self) -> &Handle<StandardMaterial> {
        match self.is_on {
            true => &self.on_material,
            false => &self.off_material,
        }
    }
}

#[derive(Component)]
struct Desk;

fn add_computer_component(
    mut commands: Commands,
    mut event_reader: EventReader<VoxelNodeReady>,
    assets: Res<AssetServer>,
) {
    for event in event_reader.read() {
        match event.name.as_str() {
            "computer" => {
                commands.entity(event.entity).insert(Computer {
                    is_on: true,
                    on_material: assets.load("study.vox#material"), // emissive texture, screen will glow
                    off_material: assets.load("study.vox#material-no-emission"), // non-emissive texture
                });
            },
            "desk" => {
                commands.entity(event.entity).insert(Desk);
            }
            _ => {},
        }
    }
}

fn toggle_computer_state(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Computer)>,
) {
    if keys.get_just_pressed().next().is_none() { return };
    let Ok((entity, mut computer)) = query.get_single_mut() else { return };
    computer.is_on = !computer.is_on;
    commands.entity(entity).insert(computer.material().clone());
}
