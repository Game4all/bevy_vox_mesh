use bevy::{ecs::{bundle::Bundle, component::Component, system::{Commands, Query, Res}, entity::Entity, event::{Event, EventWriter}}, asset::{Handle, Asset, Assets}, transform::components::Transform, reflect::TypePath, math::{Mat4, Vec3, Mat3, Quat, Vec3Swizzles}, render::{mesh::Mesh, view::Visibility, prelude::SpatialBundle}, pbr::{StandardMaterial, PbrBundle}, core::Name, hierarchy::BuildChildren, log::warn};
use dot_vox::{SceneNode, Frame};

#[derive(Bundle, Default)]
pub struct VoxelSceneBundle {
    pub scene: Handle<VoxelScene>,
    pub transform: Transform,
    pub visibility: Visibility,
}

#[derive(Asset, TypePath, Debug)]
pub struct VoxelScene {
    pub name: String,
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

#[derive(Debug, Clone)]
pub struct LayerInfo {
    pub name: Option<String>,
    pub is_hidden: bool,
}

#[derive(Component, Clone)]
pub struct VoxelLayer {
    pub id: u32,
    pub name: Option<String>,
}

#[derive(Event)]
pub struct VoxelEntityReady {
    pub scene_name: String,
    pub entity: Entity,
    pub name: String,
    pub layer_id: u32,
}

pub(crate) fn spawn_vox_scenes(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Visibility, &Handle<VoxelScene>)>,
    vox_scenes: Res<Assets<VoxelScene>>,
    mut event_writer: EventWriter<VoxelEntityReady>,
) {
    for (root, transform, visibility, scene_handle) in query.iter() {
        if let Some(scene) = vox_scenes.get(scene_handle) {
            spawn_voxel_node_recursive(&mut commands, &scene.root, root, scene, &mut event_writer);
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
    event_writer: &mut EventWriter<VoxelEntityReady>,
) {
    let mut entity_commands = commands.entity(entity);
    if let Some(model) = voxel_node.model_id.and_then(|id| {
        if let Some(model) = scene.models.get(id) { 
            Some(model) 
        } else {
            warn!("Model {} not found, omitting", id);
            None
        }
    }) {
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
            spawn_voxel_node_recursive(child_entity.commands(), child, id, scene, event_writer);
        }
    });
    if let Some(name) = &voxel_node.name {
        entity_commands.insert(Name::new(name.clone()));
        event_writer.send(VoxelEntityReady {
            scene_name: scene.name.clone(),
            entity, 
            name: name.to_string(), 
            layer_id: voxel_node.layer_id 
        });
    }
}

pub(crate) fn parse_xform_node(
    graph: &Vec<SceneNode>,
    scene_node: &SceneNode,
    parent_name: Option<&String>,
) -> VoxelNode {
    match scene_node {
        SceneNode::Transform { attributes, frames, child, layer_id } => {
            let (accumulated, node_name) = get_accumulated_and_node_name(parent_name, attributes.get("_name"));
            let mut vox_node = VoxelNode {
                name: node_name,
                transform: transform_from_frame(&frames[0]),
                is_hidden: parse_bool(attributes.get("_hidden").cloned()),
                layer_id: *layer_id,
                ..Default::default()
            };
            parse_xform_child(graph, &graph[*child as usize], &mut vox_node, accumulated.as_ref());
            vox_node                      
        }
        SceneNode::Group { .. } | SceneNode:: Shape { .. } => {
            warn!("Found Group or Shape Node without a parent Transform");
            let mut vox_node = VoxelNode::default();
            parse_xform_child(graph, scene_node, &mut vox_node, parent_name);
            vox_node
        }
    }
}

fn parse_xform_child(
    graph: &Vec<SceneNode>,
    scene_node: &SceneNode,
    partial_node: &mut VoxelNode,
    parent_name: Option<&String>,
) {
    match scene_node {
        SceneNode::Transform { .. } => {
            warn!("Found nested Transform nodes");
            partial_node.children = vec![
            parse_xform_node(graph, scene_node, parent_name)
            ];
        }
        SceneNode::Group { attributes: _, children } => {
            partial_node.children = children.iter().map(|child| {
                parse_xform_node(graph, &graph[*child as usize], parent_name)
            }).collect();
        }
        SceneNode::Shape { attributes: _, models } => {
            let model_id = models[0].model_id as usize;
            partial_node.model_id = Some(model_id);
        }
    }
}

fn get_accumulated_and_node_name(
    parent_name: Option<&String>,
    node_name: Option<&String>
) -> (Option<String>, Option<String>) {
    match (parent_name, node_name) {
        (None, None) => (None, None),
        (None, Some(node_name)) => (Some(node_name.to_string()), Some(node_name.to_string())),
        (Some(parent_name), None) => (Some(parent_name.to_string()), None), // allow group name to pass down through unnamed child
        (Some(parent_name), Some(node_name)) => {
            let accumulated = format!("{}/{}", parent_name, node_name);
            (Some(accumulated.clone()), Some(accumulated))
        },
    }
}

fn parse_bool(value: Option<String>) -> bool {
    match value.as_deref() {
        Some("1") => true,
        Some("0") => false,
        Some(_) => {
            warn!("Invalid boolean string");
            false
        },
        None => false,
    }
}

fn transform_from_frame(frame: &Frame) -> Mat4 {
    let Some(position) = frame.position() else { return Mat4::IDENTITY };
    let position = [-position.x as f32, position.z as f32, position.y as f32];
    let translation = Mat4::from_translation(Vec3::from_array(position));
    let rotation = if let Some(orientation) = frame.orientation() {
        let (rotation, scale) = &orientation.to_quat_scale();
        let scale: Vec3 = (*scale).into();
        let quat = Quat::from_array(*rotation);
        let (axis, angle) = quat.to_axis_angle();
        let mat3 = Mat3::from_axis_angle(axis.xzy(), angle) * Mat3::from_diagonal(scale);
        Mat4::from_mat3(mat3)
    } else {
        Mat4::IDENTITY
    };
    translation * rotation
}
