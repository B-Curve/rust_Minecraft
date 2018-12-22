#version 330 core

in vec4 pos0;

const int NR_LIGHTS = 32;

struct Light {
    vec3 position;
};

uniform Light lights[NR_LIGHTS];
uniform float far_plane;
void main()
{
    Light light0 = lights[0];
    float lightDistance = length(pos0.xyz - light0.position);
    lightDistance = lightDistance / far_plane;
    gl_FragDepth = lightDistance;
}