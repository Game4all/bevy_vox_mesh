use bevy::{prelude::*, asset::LoadContext};//use bevy::{asset::{Handle, LoadContext}, pbr::{StandardMaterial, PbrBundle}, render::mesh::Mesh, math::{Mat4, Mat3, Vec3, Vec4}, ecs::{system::{Commands, Query}, entity::Entity, query::Added, component::Component}, transform::components::Transform, core::Name};
use dot_vox::{SceneNode, Frame};

#[derive(Bundle)]
pub struct VoxelSceneBundle {
    pub scene: Handle<VoxelScene>,
    pub transform: Transform,
}

#[derive(Asset, TypePath)]
pub struct VoxelScene {
    pub root: VoxelNode,
    pub material: Handle<StandardMaterial>,
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
    model: Option<Handle<Mesh>>,
    is_hidden: bool,
    layer_id: u32,
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
    if let Some(model) = &voxel_node.model {
        entity_commands.insert(PbrBundle {
            mesh: model.clone(),
            material: scene.material.clone(),
            ..default()
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

pub fn parse_scene_graph(
    graph: &Vec<SceneNode>,
    scene_node: &SceneNode,
    partial_node: Option<&mut VoxelNode>,
    load_context: &mut LoadContext,
) -> Option<VoxelNode> {
    match scene_node {
        SceneNode::Transform { attributes, frames, child, layer_id } => {
            let mut vox_node = VoxelNode {
                name: attributes.get("_name").cloned(),
                transform: transform_from_frame(&frames[0]),
                children: vec![],
                model: None,
                is_hidden: parse_bool(attributes.get("_hidden").cloned()),
                layer_id: *layer_id,
            };
            parse_scene_graph(graph, &graph[*child as usize], Some(&mut vox_node), load_context);
            Some(vox_node)                             
        }
        SceneNode::Group { attributes: _, children } => {
            let Some(partial) = partial_node else { panic!("Group with no parent transform") };
            partial.children = children.iter().flat_map(|child| {
                parse_scene_graph(graph, &graph[*child as usize], None, load_context)
            }).collect();
            None
        }
        SceneNode::Shape { attributes: _, models } => {
            let Some(partial) = partial_node else { panic!("Shape with no parent transform") };
            partial.model = Some(load_context.get_label_handle(partial.name.to_owned().unwrap_or(format!("model{}", models[0].model_id))));
            None
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
    let translation = [-position.x as f32, position.z as f32, position.y as f32, 1.0];
    let Some(orientation) = frame.orientation() else { return Mat4::from_translation(Vec3::from_array(translation[0..3].try_into().expect("3 elemetns"))) };
    let mat3 = Mat3::from_cols_array_2d(&orientation.to_cols_array_2d());   
    let mut mat4 = Mat4::from_mat3(mat3);
    mat4.w_axis = Vec4::from_array(translation);
    mat4
}
