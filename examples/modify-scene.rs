use std::f32::consts::PI;
use rand::Rng;

use bevy::{prelude::*, core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping, core_3d::ScreenSpaceTransmissionQuality, experimental::taa::{TemporalAntiAliasPlugin, TemporalAntiAliasBundle}}, input::keyboard::KeyboardInput};
use bevy_vox_scene::{VoxScenePlugin, VoxelSceneBundle, VoxelEntityReady};
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

/// Uses the [`bevy_vox_scene::VoxelEntityReady`] event to add extra components into the scene graph.
/// Press any key to toggle the fish tank black-light on and off
fn main() {
    let mut app = App::new();
    
    app.add_plugins((
        DefaultPlugins,
        PanOrbitCameraPlugin,
        VoxScenePlugin,
    ))
    .init_resource::<TankSize>()
    .add_systems(Startup, setup)
    .add_systems(Update, (
        add_components_to_voxel_entities.run_if(on_event::<VoxelEntityReady>()),
        toggle_black_light.run_if(on_event::<KeyboardInput>()),
        swim_fish,
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
    
    commands.spawn(VoxelSceneBundle {
        // "tank" is the name of the group containing the glass walls, the body of water, the scenery in the tank and the fish
        scene: assets.load("study.vox#tank"),
        transform: Transform::from_scale(Vec3::splat(0.05)),
        ..default()
    });
}

#[derive(Component)]
struct EmissiveToggle {
    is_on: bool,
    on_material: Handle<StandardMaterial>,
    off_material: Handle<StandardMaterial>,
}

impl EmissiveToggle {
    fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }

    fn material(&self) -> &Handle<StandardMaterial> {
        match self.is_on {
            true => &self.on_material,
            false => &self.off_material,
        }
    }
}

#[derive(Component)]
struct Fish(f32);

#[derive(Resource, Default)]
struct TankSize(Vec3);

fn add_components_to_voxel_entities(
    mut commands: Commands,
    mut event_reader: EventReader<VoxelEntityReady>,
    mesh_query: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
    mut tank_size: ResMut<TankSize>,
    assets: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    for event in event_reader.read() {
        // If we are spawning multiple scenes we could match on the scene that the entity was spawned from. Here we just check it is the scene we're expecting.
        if event.scene_name != "study.vox#tank" { return };
        match event.name.as_str() {
            // Node names give the path to the asset, with components separated by /. Here, "black-light" belongs to the "tank" group
            "tank/black-light" => {
                commands.entity(event.entity).insert(EmissiveToggle {
                    is_on: true,
                    on_material: assets.load("study.vox#material"), // emissive texture
                    off_material: assets.load("study.vox#material-no-emission"), // non-emissive texture
                });
            },
            "tank/goldfish" | "tank/tetra" => {
                // Make fish go brrrrr
                commands.entity(event.entity).insert(Fish(rng.gen_range(5.0..10.0)));
            }
            "tank/water" => {
                // measure size of tank
                let Ok(mesh_handle) = mesh_query.get_component::<Handle<Mesh>>(event.entity) else { return };
                let Some(mesh) = meshes.get(mesh_handle) else { return };
                let Some(aabb) = mesh.compute_aabb() else { return };
                let half_extent: Vec3 = aabb.half_extents.into();
                tank_size.0 = half_extent - Vec3::splat(6.0); // add a margin
            }
            _ => {},
        }
    }
}

fn toggle_black_light(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut EmissiveToggle)>,
) {
    if keys.get_just_pressed().next().is_none() { return };
    let Ok((entity, mut emissive_toggle)) = query.get_single_mut() else { return };
    emissive_toggle.toggle();
    commands.entity(entity).insert(emissive_toggle.material().clone());
}

fn swim_fish(
    mut query: Query<(&mut Transform, &Fish)>,
    time: Res<Time>,
    tank_size: Res<TankSize>,
) {
    for (mut transform, fish) in query.iter_mut() {
        let x_direction = transform.forward().dot(Vec3::X);
        if (x_direction < -0.5 && transform.translation.x < -tank_size.0.x) || (x_direction > 0.5 && transform.translation.x > tank_size.0.x) {
            // change direction at tank edges
            transform.rotate_axis(Vec3::Y, PI);
        }
        // slow down when near the edge
        let slow_down = 1.0 - ((transform.translation.x.abs() - (tank_size.0.x - 4.2)) / 5.0).clamp(0.0, 1.0);
        let forward = transform.forward();
        transform.translation += forward * (time.delta_seconds() * fish.0 * slow_down);
        // make them weave up and down
        transform.translation.y = (transform.translation.x * 0.1).sin() * 6.0;
    }
}
