#version 330 core
out vec4 color;

in vec2 uv0;

uniform sampler2D gPosition;
uniform sampler2D gNormal;
uniform sampler2D gAlbedoSpec;

struct Light {
    vec3 position;
    vec3 color;

    float linear;
    float quadratic;
    float radius;
};
const int NR_LIGHTS = 32;
uniform Light lights[NR_LIGHTS];
uniform vec3 viewPos;
void main()
{
    vec3 pos = texture(gPosition, uv0).rgb;
    vec3 normal = texture(gNormal, uv0).rgb;
    vec3 albedo = texture(gAlbedoSpec, uv0).rgb;

    vec3 lighting = albedo * 0.1;
    vec3 viewDir = normalize(viewPos - pos);
    for (int i = 0; i < NR_LIGHTS; ++i)
    {
        float distance = length(lights[i].position - pos);
        if (distance < lights[i].radius)
        {
            vec3 lightDir = normalize(lights[i].position - pos);
            vec3 diff = max(dot(normal, lightDir), 0.0) * albedo * lights[i].color;
            float attenuation = 1.0 / (1.0 + lights[i].linear * distance + lights[i].quadratic * distance * distance);
            diff *= attenuation;
            lighting += diff;
        }
    }

    color = vec4(lighting, 1.0);
}