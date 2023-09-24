use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_vox_mesh::{vox_scene_info::VoxSceneInfo, VoxMeshPlugin};
use std::f32::consts::PI;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(VoxMeshPlugin::default())
        .register_type::<Entity>()
        .insert_resource(BoyMate {
            handle: None,
            mate: None,
        })
        .insert_resource(BoyEntity { boy_entity: None })
        .insert_resource(FaceNow::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (load_mate, load_boy, toggle_faces))
        .run();
}

#[derive(Debug, Resource)]
pub struct BoyEntity {
    pub boy_entity: Option<Entity>,
}

#[derive(Debug, Resource, Clone)]
pub struct BoyMate {
    pub handle: Option<Handle<VoxSceneInfo>>,
    pub mate: Option<VoxSceneInfo>,
}

#[derive(Debug, Resource, Clone)]
pub struct FaceNow {
    pub now_face: &'static str,
}

impl Default for FaceNow {
    fn default() -> Self {
        Self { now_face: "face0" }
    }
}

fn toggle_faces(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &Name, &mut Visibility)>,
    mut face_now: ResMut<FaceNow>,
) {
    let faces = vec!["face0", "face1", "face2", "face3"];
    if keyboard_input.just_pressed(KeyCode::Tab) {
        if let Some(index) = faces.iter().position(|&x| x == face_now.now_face) {
            let next_index = if index == faces.len() - 1 {
                0
            } else {
                index + 1
            };
            let next_face = faces[next_index];
            for (_, name, mut visibility) in query.iter_mut() {
                if faces.contains(&name.as_str()) {
                    if name.as_str() == next_face {
                        *visibility.as_mut() = Visibility::Inherited;
                    } else {
                        *visibility.as_mut() = Visibility::Hidden;
                    }
                }
            }
            face_now.now_face = next_face;
        }
    }
}

fn load_boy(
    mut commands: Commands,
    boy_mate: Res<BoyMate>,
    mut boy_entity: ResMut<BoyEntity>,
    assets: Res<AssetServer>,
    mut stdmats: ResMut<Assets<StandardMaterial>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    if let Some(_entity) = boy_entity.boy_entity {
        // 这里可以进行其他的处理?
    } else {
        if let Some(mate_data) = boy_mate.mate.clone() {
            if mate_data.all_loaded("boy.vox", mesh_assets.as_ref(), assets.as_ref()) {
                // println!("这里生成模型的详情");
                let boy = mate_data.to_entity(
                    "boy.vox",
                    &mut commands,
                    assets.as_ref(),
                    stdmats.add(Color::rgb(1., 1., 1.).into()),
                    &mut mesh_assets,
                );
                commands.entity(boy).insert((
                    Visibility::Inherited,
                    ComputedVisibility::HIDDEN,
                    GlobalTransform::IDENTITY,
                    Transform {
                        translation: Vec3 {
                            x: 0.0,
                            y: 1.0 / 40. * 40., // height is 80 so the button is scale*80/2
                            z: 0.0,
                        },
                        scale: Vec3 {
                            x: 1.0 / 40.,
                            y: 1.0 / 40.,
                            z: 1.0 / 40.,
                        },
                        ..Default::default()
                    } * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
                ));
                boy_entity.boy_entity = Some(boy);
            }
        }
    }
}

fn load_mate(mate_assets: Res<Assets<VoxSceneInfo>>, mut boy_mate: ResMut<BoyMate>) {
    if let Some(handle) = boy_mate.handle.clone() {
        match boy_mate.mate {
            Some(_) => {}
            None => {
                if let Some(mate) = mate_assets.get(&handle) {
                    boy_mate.mate = Some(mate.clone());
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stdmats: ResMut<Assets<StandardMaterial>>,
    mut boy_mate: ResMut<BoyMate>,
    assets: Res<AssetServer>,
) {
    let mate_data_handle: Handle<VoxSceneInfo> = assets.load("boy.vox#mate_data");
    boy_mate.handle = Some(mate_data_handle);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            subdivisions: 2,
            size: 5.0,
        })),
        material: stdmats.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // commands.spawn(PbrBundle {
    //     transform: Transform::from_scale((0.01, 0.01, 0.01).into())
    //         * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
    //     mesh: assets.load("boy.vox#model5"),
    //     material: stdmats.add(Color::rgb(1., 1., 1.).into()),
    //     ..Default::default()
    // });
}
