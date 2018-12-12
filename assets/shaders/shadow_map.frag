#version 330 core

in vec4 pos0;

uniform vec3 lightPos;
uniform float far_plane;
void main()
{
    float lightDistance = length(pos0.xyz - lightPos);
    lightDistance = lightDistance / far_plane;
    gl_FragDepth = lightDistance;
}