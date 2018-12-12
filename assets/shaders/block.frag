#version 330 core
layout (location = 0) out vec3 gPosition;
layout (location = 1) out vec3 gNormal;
layout (location = 2) out vec4 gAlbedoSpec;

in vec3 pos0;
in vec3 normal0;
in vec2 uv0;

uniform sampler2D tex;
void main()
{
    gPosition = pos0;
    gNormal = normalize(normal0);
    gAlbedoSpec.rgb = texture(tex, uv0).rgb;
    gAlbedoSpec.a = texture(tex, uv0).a;
}