use bevy::{ecs::{bundle::Bundle, component::Component, system::{Commands, Query, Res}, entity::Entity}, asset::{Handle, Asset, Assets}, transform::components::Transform, reflect::TypePath, math::{Mat4, Vec3, Mat3}, render::{mesh::Mesh, view::Visibility, prelude::SpatialBundle}, pbr::{StandardMaterial, PbrBundle}, core::Name, hierarchy::BuildChildren};
use dot_vox::{SceneNode, Frame};

#[derive(Bundle)]
pub struct VoxelSceneBundle {
    pub scene: Handle<VoxelScene>,
    pub transform: Transform,
}

#[derive(Asset, TypePath)]
pub struct VoxelScene {
    pub root: VoxelNode,
    pub models: Vec<VoxelModel>,
    pub layers: Vec<LayerInfo>,
}

pub struct LayerInfo {
    pub name: Option<String>,
    pub is_hidden: bool,
}

#[derive(Debug)]
pub struct VoxelNode {
    name: Option<String>,
    transform: Mat4,
    children: Vec<VoxelNode>,
    model_id: Option<usize>,
    is_hidden: bool,
    layer_id: u32,
}

#[derive(Debug)]
pub struct VoxelModel {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Component, Clone)]
pub struct VoxelLayer {
    pub id: u32,
    pub name: Option<String>,
}

pub fn spawn_vox_scenes(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Handle<VoxelScene>)>,
    vox_scenes: Res<Assets<VoxelScene>>,
) {
    for (root, transform, scene_handle) in query.iter() {
        if let Some(scene) = vox_scenes.get(scene_handle) {
            spawn_voxel_node_recursive(&mut commands, &scene.root, root, &scene);
            commands.entity(root)
            .remove::<Handle<VoxelScene>>()
            .insert(*transform);
        }
    }
}

fn spawn_voxel_node_recursive(
    commands: &mut Commands,
    voxel_node: &VoxelNode,
    entity: Entity,
    scene: &VoxelScene,
) {
    let mut entity_commands = commands.entity(entity);
    if let Some(name) = &voxel_node.name {
        entity_commands.insert(Name::new(name.clone()));
    }
    if let Some(model_id) = voxel_node.model_id {
        let Some(model) = scene.models.get(model_id) else { panic!("Model not found") };
        entity_commands.insert(PbrBundle {
            mesh: model.mesh.clone(),
            material: model.material.clone(),
            ..Default::default()
        });
    } else {
        entity_commands.insert(SpatialBundle::default());
    }
    
    if let Some(layer_info) = scene.layers.get(voxel_node.layer_id as usize) {
        entity_commands.insert((
            VoxelLayer {
                id: voxel_node.layer_id,
                name: layer_info.name.clone(),
            },
            if voxel_node.is_hidden || layer_info.is_hidden { Visibility::Hidden } else { Visibility::Inherited },
        ));
    }
    entity_commands.insert(
        Transform::from_matrix(voxel_node.transform),
    ).with_children(|builder| {
        for child in &voxel_node.children {
            let mut child_entity = builder.spawn_empty();
            let id = child_entity.id();
            spawn_voxel_node_recursive(child_entity.commands(), &child, id, scene);
        }
    });
}

pub fn parse_xform_node(
    graph: &Vec<SceneNode>,
    scene_node: &SceneNode,
    shape_names: &mut Vec<Option<String>>,
) -> VoxelNode {
    match scene_node {
        SceneNode::Transform { attributes, frames, child, layer_id } => {
            let mut vox_node = VoxelNode {
                name: attributes.get("_name").cloned(),
                transform: transform_from_frame(&frames[0]),
                children: vec![],
                model_id: None,
                is_hidden: parse_bool(attributes.get("_hidden").cloned()),
                layer_id: *layer_id,
            };
            parse_xform_child(graph, &graph[*child as usize], &mut vox_node, shape_names);
            vox_node                        
        }
        SceneNode::Group { .. } | SceneNode:: Shape { .. } => { panic!("expected Transform node") }
    }
}

fn parse_xform_child(
    graph: &Vec<SceneNode>,
    scene_node: &SceneNode,
    partial_node: &mut VoxelNode,
    shape_names: &mut Vec<Option<String>>,
) {
    match scene_node {
        SceneNode::Transform { .. } => { panic!("expected Group or Shape node") }
        SceneNode::Group { attributes: _, children } => {
            partial_node.children = children.iter().map(|child| {
                parse_xform_node(graph, &graph[*child as usize], shape_names)
            }).collect();
        }
        SceneNode::Shape { attributes: _, models } => {
            let model_id = models[0].model_id as usize;
            partial_node.model_id = Some(model_id);
            if let Some(existing_name) = &shape_names[model_id] {
                if existing_name.starts_with("model-") {
                    if let Some(parent_name) = &partial_node.name {
                        // overwrite anonymous "model-" name if better alternative exists
                        disambiguate_name_and_add(parent_name, model_id, shape_names);
                    }
                }
                // existing shape, ignore name of parent xform
            } else if let Some(parent_name) = &partial_node.name {
                disambiguate_name_and_add(parent_name, model_id, shape_names);
            } else {
                // disambiguated anonymous name
                shape_names[model_id] = Some(format!("model-{}", model_id));
            }
        }
    }
}

fn disambiguate_name_and_add(
    parent_name: &String,
    model_id: usize,
    shape_names: &mut Vec<Option<String>>
) {
    if shape_names.contains(&Some(parent_name.to_string())) || parent_name.is_empty() {
         // disambiguate name by appending model id
         shape_names[model_id] = Some(format!("{}-{}", parent_name, model_id));
    } else {
        shape_names[model_id] = Some(parent_name.to_string());
    }
}

fn parse_bool(value: Option<String>) -> bool {
    match value.as_deref() {
        Some("1") => true,
        Some(_) => false,
        None => false,
    }
}

fn transform_from_frame(frame: &Frame) -> Mat4 {
    let Some(position) = frame.position() else { return Mat4::IDENTITY };
    let position = [-position.x as f32, position.z as f32, position.y as f32];
    let translation = Mat4::from_translation(Vec3::from_array(position));
    let rotation = if let Some(orientation) = frame.orientation() {
        let mat3 = Mat3::from_cols_array_2d(&orientation.to_cols_array_2d());
        Mat4::from_mat3(mat3)
    } else {
        Mat4::IDENTITY
    };
    translation * rotation
}
