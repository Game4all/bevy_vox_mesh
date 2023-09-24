use bevy::{
    prelude::{
        AssetServer, Assets, BuildChildren, Commands, Component, ComputedVisibility, Entity,
        GlobalTransform, Handle, Mesh, Name, PbrBundle, StandardMaterial, Transform, Visibility,
    },
    reflect::{TypePath, TypeUuid},
    utils::HashMap,
};
use dot_vox::{Layer, SceneNode};

#[derive(Debug, TypeUuid, TypePath, Clone)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]

pub struct VoxSceneInfo {
    pub scenes: Vec<SceneNode>,
    pub layers: Vec<Layer>,
    // layers hidden status
    pub layer_map: HashMap<u32, bool>,
}

#[derive(Debug, Clone, Component)]
pub struct LayerData(pub u32);

impl VoxSceneInfo {
    pub fn new(scenes: Vec<SceneNode>, layers: Vec<Layer>) -> Self {
        Self {
            scenes: scenes,
            layers: layers.clone(),
            layer_map: layers
                .clone()
                .into_iter()
                .map(|x| {
                    let mut number: u32 = u32::MAX;
                    if let Some(s) = x.attributes.get("_name") {
                        number = s.parse().unwrap();
                    }
                    let mut hidden = false;
                    if let Some(s) = x.attributes.get("_hidden") {
                        if s == "1" {
                            hidden = true;
                        }
                    }
                    (number, hidden)
                })
                .collect(),
        }
    }

    pub fn all_loaded(
        &self,
        base_id: &'static str,
        mesh_assets: &Assets<Mesh>,
        asset_server: &AssetServer,
    ) -> bool {
        for node in self.scenes.iter() {
            match node {
                SceneNode::Shape {
                    attributes: _,
                    models,
                } => {
                    for shape in models {
                        let key = if shape.model_id == 0 {
                            format!("{}", base_id,)
                        } else {
                            format!("{}#model{}", base_id, shape.model_id)
                        };
                        let handle: Handle<Mesh> = asset_server.get_handle(key);
                        if mesh_assets.get(&handle).is_none() {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }
        return true;
    }

    pub fn to_entity(
        &self,
        base_id: &'static str,
        commands: &mut Commands,
        asset_server: &AssetServer,
        material_handle: Handle<StandardMaterial>,
        mesh_assets: &mut Assets<Mesh>,
    ) -> Entity {
        // 这里要根据 sence的定义生成一系列的entity
        let root_scene = &self.scenes[0];
        let ret = deal_scene_node(
            base_id,
            commands,
            asset_server,
            root_scene,
            &self.scenes,
            material_handle.clone(),
            mesh_assets,
            &self.layer_map,
        );
        ret[0]
    }
}

fn deal_scene_node(
    base_id: &'static str,
    commands: &mut Commands,
    asset_server: &AssetServer,
    scene_node: &SceneNode,
    scenes_tree: &Vec<SceneNode>,
    material_handle: Handle<StandardMaterial>,
    mesh_assets: &mut Assets<Mesh>,
    layer_map: &HashMap<u32, bool>,
) -> Vec<Entity> {
    let mut result: Vec<Entity> = Vec::new();
    match scene_node {
        SceneNode::Transform {
            attributes,
            frames,
            child,
            layer_id,
        } => {
            // 标记一下当前数据？
            let mut node = commands.spawn(LayerData(layer_id.clone()));
            if let Some(name) = attributes.get("_name") {
                node.insert(Name::new(name.to_owned()));
            }
            for frame in frames.iter() {
                // TODO: Support Other Types
                if let Some(pos) = frame.position() {
                    node.insert(Transform::from_xyz(
                        pos.x as f32,
                        pos.y as f32,
                        pos.z as f32,
                    ));
                }
            }

            let children = deal_scene_node(
                base_id,
                node.commands(),
                asset_server,
                &scenes_tree[child.clone() as usize],
                scenes_tree,
                material_handle.clone(),
                mesh_assets,
                layer_map,
            );
            node.push_children(&children);

            let visibilty = if let Some(hidden) = layer_map.get(layer_id) {
                if *hidden {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                }
            } else {
                Visibility::Inherited
            };
            node.insert((
                visibilty,
                ComputedVisibility::HIDDEN,
                GlobalTransform::IDENTITY,
            ));
            result.push(node.id());
        }
        SceneNode::Group {
            attributes: _,
            children,
        } => {
            // 获取一组数据
            for ch_key in children {
                let children = deal_scene_node(
                    base_id,
                    commands,
                    asset_server,
                    &scenes_tree[ch_key.clone() as usize],
                    scenes_tree,
                    material_handle.clone(),
                    mesh_assets,
                    layer_map,
                );
                result.extend(children);
            }
        }
        SceneNode::Shape {
            attributes: _,
            models,
        } => {
            // 这里生成单个的entity
            for shape in models {
                let key = if shape.model_id == 0 {
                    format!("{}", base_id,)
                } else {
                    format!("{}#model{}", base_id, shape.model_id)
                };
                let handle: Handle<Mesh> = asset_server.get_handle(key.clone());
                println!("{}-{:?}", key, asset_server.get_load_state(handle.clone()));
                if let Some(mesh) = mesh_assets.get(&handle) {
                    result.push(
                        commands
                            .spawn(PbrBundle {
                                transform: Transform::IDENTITY,
                                mesh: mesh_assets.add(mesh.clone()),
                                material: material_handle.clone(),
                                ..Default::default()
                            })
                            .id(),
                    );
                }
            }
        }
    }
    result
}
