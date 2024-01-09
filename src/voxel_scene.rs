use bevy::{ecs::{bundle::Bundle, component::Component, system::{Commands, Query, Res}, entity::Entity}, asset::{Handle, Asset, Assets}, transform::components::Transform, reflect::TypePath, math::{Mat4, Vec3, Mat3}, render::{mesh::Mesh, view::Visibility, prelude::SpatialBundle}, pbr::{StandardMaterial, PbrBundle}, core::Name, hierarchy::BuildChildren};
use dot_vox::{SceneNode, Frame};

#[derive(Bundle, Default)]
pub struct VoxelSceneBundle {
    pub scene: Handle<VoxelScene>,
    pub transform: Transform,
    pub visibility: Visibility,
}

#[derive(Asset, TypePath)]
pub struct VoxelScene {
    pub root: VoxelNode,
    pub models: Vec<VoxelModel>,
    pub layers: Vec<LayerInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct VoxelNode {
    pub name: Option<String>,
    pub transform: Mat4,
    pub children: Vec<VoxelNode>,
    pub model_id: Option<usize>,
    pub is_hidden: bool,
    pub layer_id: u32,
}

#[derive(Debug, Clone)]
pub struct VoxelModel {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Clone)]
pub struct LayerInfo {
    pub name: Option<String>,
    pub is_hidden: bool,
}

#[derive(Component, Clone)]
pub struct VoxelLayer {
    pub id: u32,
    pub name: Option<String>,
}

pub(crate) fn spawn_vox_scenes(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Visibility, &Handle<VoxelScene>)>,
    vox_scenes: Res<Assets<VoxelScene>>,
) {
    for (root, transform, visibility, scene_handle) in query.iter() {
        if let Some(scene) = vox_scenes.get(scene_handle) {
            spawn_voxel_node_recursive(&mut commands, &scene.root, root, scene);
            commands.entity(root)
            .remove::<Handle<VoxelScene>>()
            .insert((*transform, *visibility));
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
            spawn_voxel_node_recursive(child_entity.commands(), child, id, scene);
        }
    });
}

pub(crate) fn parse_xform_node(
    graph: &Vec<SceneNode>,
    scene_node: &SceneNode,
) -> VoxelNode {
    match scene_node {
        SceneNode::Transform { attributes, frames, child, layer_id } => {
            let mut vox_node = VoxelNode {
                name: attributes.get("_name").cloned(),
                transform: transform_from_frame(&frames[0]),
                is_hidden: parse_bool(attributes.get("_hidden").cloned()),
                layer_id: *layer_id,
                ..Default::default()
            };
            parse_xform_child(graph, &graph[*child as usize], &mut vox_node);
            vox_node                        
        }
        SceneNode::Group { .. } | SceneNode:: Shape { .. } => { panic!("expected Transform node") }
    }
}

fn parse_xform_child(
    graph: &Vec<SceneNode>,
    scene_node: &SceneNode,
    partial_node: &mut VoxelNode,
) {
    match scene_node {
        SceneNode::Transform { .. } => { panic!("expected Group or Shape node") }
        SceneNode::Group { attributes: _, children } => {
            partial_node.children = children.iter().map(|child| {
                parse_xform_node(graph, &graph[*child as usize])
            }).collect();
        }
        SceneNode::Shape { attributes: _, models } => {
            let model_id = models[0].model_id as usize;
            partial_node.model_id = Some(model_id);
        }
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
