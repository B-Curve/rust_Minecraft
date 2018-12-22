#version 330 core
layout (triangles) in;
layout (triangle_strip, max_vertices=18) out;

const int NR_LIGHTS = 32;

struct Transform {
    mat4 t[6];
};

uniform Transform shadowTransforms[NR_LIGHTS];

out vec4 pos0;

void main()
{
    mat4 t1[6] = shadowTransforms[0].t;
    for (int face = 0; face < 6; face++)
    {
        gl_Layer = face;
        for (int i = 0; i < 3; i++)
        {
            pos0 = gl_in[i].gl_Position;
            gl_Position = t1[face] * pos0;
            EmitVertex();
        }
        EndPrimitive();
    }
}