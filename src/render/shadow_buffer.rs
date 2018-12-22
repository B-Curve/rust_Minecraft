use GL;
use GL::Gl;
use math::{Mat4, Ext::{perspective, look_at}};
use std::ops::Index;
use world::block::block_light::BlockLight;
use shader::{Shader, Type as ShaderType};
use window::Window;

const SHADOW_SIZE: i32 = 1024;
const NEAR_PLANE: f32 = 1.0;
const FAR_PLANE: f32 = 25.0;

struct ShadowTransform {
    right: Mat4,
    left: Mat4,
    top: Mat4,
    bottom: Mat4,
    back: Mat4,
    front: Mat4
}

impl ShadowTransform {
    pub fn new(right: Mat4, left: Mat4, top: Mat4, bottom: Mat4, back: Mat4, front: Mat4) -> ShadowTransform {
        ShadowTransform {
            right, left, top, bottom, back, front
        }
    }

    pub fn len(&self) -> u32 { 6 }
}

impl Index<u32> for ShadowTransform {
    type Output = Mat4;
    fn index(&self, index: u32) -> &Mat4 {
        match index {
            0 => &self.right,
            1 => &self.left,
            2 => &self.top,
            3 => &self.bottom,
            4 => &self.back,
            5 => &self.front,
            _ => panic!("Index out of bounds"),
        }
    }
}

pub struct ShadowBuffer {
    gl: Gl,
    depth_map_fbo: u32,
    depth_cube_map: u32,
    shader: Shader,
    transforms: Vec<ShadowTransform>
}

impl ShadowBuffer {
    pub fn new(gl: &Gl) -> ShadowBuffer {
        let (mut depth_map_fbo, mut depth_cube_map) = (0, 0);

        unsafe {
            gl.GenFramebuffers(1, &mut depth_map_fbo);
            gl.GenTextures(1, &mut depth_cube_map);
            gl.BindTexture(GL::TEXTURE_CUBE_MAP, depth_cube_map);

            for i in 0..6 {
                gl.TexImage2D(GL::TEXTURE_CUBE_MAP_POSITIVE_X + i, 0, GL::DEPTH_COMPONENT as i32, SHADOW_SIZE, SHADOW_SIZE, 0, GL::DEPTH_COMPONENT, GL::FLOAT, null!())
            }
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_WRAP_R, GL::CLAMP_TO_EDGE as i32);
            gl.BindFramebuffer(GL::FRAMEBUFFER, depth_map_fbo);
            gl.FramebufferTexture(GL::FRAMEBUFFER, GL::DEPTH_ATTACHMENT, depth_cube_map, 0);
            gl.DrawBuffer(GL::NONE);
            gl.ReadBuffer(GL::NONE);
            gl.BindFramebuffer(GL::FRAMEBUFFER, 0);
        }

        ShadowBuffer {
            gl: gl.clone(), depth_map_fbo, depth_cube_map,
            shader: Shader::new(gl, ShaderType::ShadowMap, true).unwrap(),
            transforms: Vec::new()
        }
    }

    pub fn gen_shadows(&mut self, lights: Vec<&BlockLight>) {
        let shadow_proj = perspective(90f32.to_radians(), 1f32, NEAR_PLANE, FAR_PLANE);
        self.shader.bind();
        self.shader.float("far_plane", FAR_PLANE);
        for (index, light) in lights.iter().enumerate() {
            let p = light.position.clone();
            let t = ShadowTransform::new(
                shadow_proj * look_at(p, p + vector!(1.0, 0.0, 0.0), vector!(0.0, -1.0, 0.0)),
                shadow_proj * look_at(p, p + vector!(-1.0, 0.0, 0.0), vector!(0.0, -1.0, 0.0)),
                shadow_proj * look_at(p, p + vector!(0.0, 1.0, 0.0), vector!(0.0, 0.0, 1.0)),
                shadow_proj * look_at(p, p + vector!(0.0, -1.0, 0.0), vector!(0.0, 0.0, -1.0)),
                shadow_proj * look_at(p, p + vector!(0.0, 0.0, 1.0), vector!(0.0, -1.0, 0.0)),
                shadow_proj * look_at(p, p + vector!(0.0, 0.0, -1.0), vector!(0.0, -1.0, 0.0)),
            );
            for i in 0..t.len() {
                self.shader.mat_4(&format!("shadowTransforms[{}].t[{}]", index, i), &t[i]);
            }
            self.shader.vec3(&format!("lights[{}].position", index), &light.position);
            self.transforms.push(t);
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.Viewport(0, 0, SHADOW_SIZE, SHADOW_SIZE);
            self.gl.BindFramebuffer(GL::FRAMEBUFFER, self.depth_map_fbo);
            self.gl.Clear(GL::DEPTH_BUFFER_BIT);
            self.shader.bind();
        }
    }

    pub fn unbind(&self, window: &Window) {
        unsafe {
            self.gl.BindFramebuffer(GL::FRAMEBUFFER, 0);
            let (x, y) = window.size();
            self.gl.Viewport(0, 0, x, y);
            self.gl.Clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        }
    }
}