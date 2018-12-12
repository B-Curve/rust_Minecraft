#version 330 core

out vec4 color;

in vec2 uv0;

uniform vec3 lightColor;
uniform sampler2D tex;
void main()
{
    color = texture(tex, uv0);
}