use bevy::{
    prelude::*,
    render::{
        pipeline::{PipelineDescriptor, RenderPipeline},
        shader::{ShaderStage, ShaderStages},
    },
};
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

vec4 toLinear(vec4 sRGB)
{
    bvec4 cutoff = lessThan(sRGB, vec4(0.04045));
    vec4 higher = pow((sRGB + vec4(0.055))/vec4(1.055), vec4(2.4));
    vec4 lower = sRGB/vec4(12.92);

    return mix(higher, lower, cutoff);
}

void main() {
    o_Target = toLinear(vertex_color);
}"#;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(VoxMeshPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: ResMut<AssetServer>,
) {
    let handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, BASIC_COLOR_VERT)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, BASIC_COLOR_FRAG))),
    }));

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    let mesh = assets.load("../examples/chicken.vox");

    commands.spawn_bundle(MeshBundle {
        mesh: mesh,
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(handle)]),
        transform: Transform::from_scale((0.01, 0.01, 0.01).into()),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
