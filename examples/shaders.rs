pub const BASIC_COLOR_VERT: &str = r#"#version 450

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

pub const BASIC_COLOR_FRAG: &str = r#"#version 450

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

#[allow(dead_code)]
fn main() {}
