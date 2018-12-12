#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;
layout (location = 2) in vec3 normal;

out vec2 uv0;
out vec3 normal0;
out vec3 pos0;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
void main()
{
    vec4 worldPos = model * vec4(pos, 1.0);
    pos0 = worldPos.xyz;
    uv0 = uv;

    mat3 normalMatrix = transpose(inverse(mat3(model)));
    normal0 = normalMatrix * normal;

    gl_Position = projection * view * worldPos;
}