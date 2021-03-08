#version 450

layout(location=0) in vec2 v_Uv;
layout(location=0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 1) uniform texture2D SkyMaterial_texture;
layout(set = 2, binding = 2) uniform sampler SkyMaterial_texture_sampler;


void main() {
    o_Target = texture(sampler2D(SkyMaterial_texture, SkyMaterial_texture_sampler), v_Uv);
}

