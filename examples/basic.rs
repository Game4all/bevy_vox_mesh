use bevy::prelude::*;
use bevy_vox_mesh::VoxMeshPlugin;

const BASIC_COLOR_VERT: &str = r#"#version 450

layout(location = 0) in vec4 Vertex_Position;
layout(location = 1) in vec4 Vertex_Color;
layout(location = 0) out vec4 vertex_color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    gl_Position = ViewProj * Model * Vertex_Position;
    vertex_color = Vertex_Color;
}"#;

const BASIC_COLOR_FRAG: &str = r#"#version 450

layout(location = 0) in vec4 vertex_color;
layout(location = 0) out vec4 o_Target;

void main() {
    o_Target = vertex_color;
}"#;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(VoxMeshPlugin)
        .run();
}

fn setup() {
    
}
