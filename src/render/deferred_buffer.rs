use GL;
use GL::Gl;
use shader::{Shader, Type as ShaderType};
use math::{Mat4, Vec3, One, Ext::{translate, scale}};
use std::mem::size_of;
use std::ptr::null as NULL;
use std::ffi::c_void;
use world::block::block_light::BlockLight;
use camera::Camera;
use world::block::block::Block;
use world::block::block_buffer::BlockBuffer;
use world::block::block_database;

lazy_static! {
    static ref quad: [f32; 20] = [
        -1.0, 1.0, 0.0, 0.0, 1.0,
        -1.0, -1.0, 0.0, 0.0, 0.0,
        1.0, 1.0, 0.0, 1.0, 1.0,
        1.0, -1.0, 0.0, 1.0, 0.0
    ];
}

const CONSTANT: f32 = 1.0;
const LINEAR: f32 = 0.7;
const QUADRATIC: f32 = 1.8;

pub struct DeferredBuffer {
    gl: Gl,
    g_buffer: u32,
    g_position: u32,
    g_normal: u32,
    g_albedo: u32,
    rbo_depth: u32,
    vao: u32,
    vbo: u32,
    shader: Shader,
    framebuffer_size: (i32, i32)
}

impl DeferredBuffer {
    pub fn new(gl: &Gl, framebuffer_size: (i32, i32)) -> DeferredBuffer {
        let (width, height) = framebuffer_size;
        let (mut g_buffer, mut g_position, mut g_normal, mut g_albedo, mut rbo_depth, mut vao, mut vbo) = (0, 0, 0, 0, 0, 0, 0);

        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);
            gl.BindVertexArray(vao);
            gl.BindBuffer(GL::ARRAY_BUFFER, vbo);
            gl.BufferData(GL::ARRAY_BUFFER, (size_of::<f32>() * quad.len()) as isize, quad.as_ptr() as *const c_void, GL::STATIC_DRAW);
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(0, 3, GL::FLOAT, GL::FALSE, (5 * size_of::<f32>()) as i32, NULL());
            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(1, 2, GL::FLOAT, GL::FALSE, (5 * size_of::<f32>()) as i32, (3 * size_of::<f32>()) as *const c_void);
            gl.BindVertexArray(0);
            gl.BindBuffer(GL::ARRAY_BUFFER, 0);

            gl.GenFramebuffers(1, &mut g_buffer);
            gl.BindFramebuffer(GL::FRAMEBUFFER, g_buffer);

            gl.GenTextures(1, &mut g_position);
            gl.BindTexture(GL::TEXTURE_2D, g_position);
            gl.TexImage2D(GL::TEXTURE_2D, 0, GL::RGB16F as i32, width, height, 0, GL::RGB, GL::FLOAT, ::std::ptr::null());
            gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
            gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
            gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
            gl.FramebufferTexture2D(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT0, GL::TEXTURE_2D, g_position, 0);

            gl.GenTextures(1, &mut g_normal);
            gl.BindTexture(GL::TEXTURE_2D, g_normal);
            gl.TexImage2D(GL::TEXTURE_2D, 0, GL::RGB16F as i32, width, height, 0, GL::RGB, GL::FLOAT, ::std::ptr::null());
            gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
            gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
            gl.FramebufferTexture2D(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT1, GL::TEXTURE_2D, g_normal, 0);

            gl.GenTextures(1, &mut g_albedo);
            gl.BindTexture(GL::TEXTURE_2D, g_albedo);
            gl.TexImage2D(GL::TEXTURE_2D, 0, GL::RGBA as i32, width, height, 0, GL::RGBA, GL::UNSIGNED_BYTE, ::std::ptr::null());
            gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
            gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
            gl.FramebufferTexture2D(GL::FRAMEBUFFER, GL::COLOR_ATTACHMENT2, GL::TEXTURE_2D, g_albedo, 0);

            let attachments: [u32; 3] = [GL::COLOR_ATTACHMENT0, GL::COLOR_ATTACHMENT1, GL::COLOR_ATTACHMENT2];
            gl.DrawBuffers(3, attachments.as_ptr());

            gl.GenRenderbuffers(1, &mut rbo_depth);
            gl.BindRenderbuffer(GL::RENDERBUFFER, rbo_depth);
            gl.RenderbufferStorage(GL::RENDERBUFFER, GL::DEPTH_COMPONENT, width, height);
            gl.FramebufferRenderbuffer(GL::FRAMEBUFFER, GL::DEPTH_ATTACHMENT, GL::RENDERBUFFER, rbo_depth);
            if gl.CheckFramebufferStatus(GL::FRAMEBUFFER) != GL::FRAMEBUFFER_COMPLETE {
                println!("Framebuffer not complete!");
            }
            gl.BindFramebuffer(GL::FRAMEBUFFER, 0);
        }

        let shader = Shader::new(gl, ShaderType::Deferred, false).unwrap();
        shader.bind();
        shader.int("gPosition", 0);
        shader.int("gNormal", 1);
        shader.int("gAlbedoSpec", 2);

        DeferredBuffer {
            gl: gl.clone(), g_buffer, g_position,
            g_normal, g_albedo, rbo_depth, vao, vbo, shader, framebuffer_size
        }
    }

    pub fn shader(&self) -> &Shader {
        &self.shader
    }

    fn draw_quad(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
            self.gl.DrawArrays(GL::TRIANGLE_STRIP, 0, 4);
            self.gl.BindVertexArray(0);
        }
    }

    pub fn bind_framebuffer(&self) {
        unsafe {
            self.gl.BindFramebuffer(GL::FRAMEBUFFER, self.g_buffer);
            self.gl.Clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        }
    }

    pub fn unbind_framebuffer(&self) {
        unsafe {
            self.gl.BindFramebuffer(GL::FRAMEBUFFER, 0);
            self.gl.Clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        }
    }

    pub fn apply_lighting(&self, view_pos: &Vec3, lights: &Vec<&BlockLight>) {
        self.shader.bind();
        unsafe {
            self.gl.ActiveTexture(GL::TEXTURE0);
            self.gl.BindTexture(GL::TEXTURE_2D, self.g_position);
            self.gl.ActiveTexture(GL::TEXTURE1);
            self.gl.BindTexture(GL::TEXTURE_2D, self.g_normal);
            self.gl.ActiveTexture(GL::TEXTURE2);
            self.gl.BindTexture(GL::TEXTURE_2D, self.g_albedo);
        }
        for (i, light) in lights.iter().enumerate() {
            let l = (7.0 / light.strength) * LINEAR;
            let q = (7.0 / light.strength) * QUADRATIC;
            self.shader.vec3(&format!("lights[{}].position", i), &light.position);
            self.shader.vec3(&format!("lights[{}].color", i), &light.color);
            self.shader.float(&format!("lights[{}].linear", i), l);
            self.shader.float(&format!("lights[{}].quadratic", i), q);
            let brightness = (light.color.x.max(light.color.y)).max(light.color.z);
            let radius = (-l + (l * l - 4.0 * q * (CONSTANT - (256.0 / 5.0) * brightness)).sqrt()) / (2.0 * q);
            self.shader.float(&format!("lights[{}].radius", i), radius);
        }
        self.shader.vec3("viewPos", view_pos);
        self.draw_quad();
    }

    pub fn copy_depth_buffer(&self) {
        unsafe {
            self.gl.BindFramebuffer(GL::READ_FRAMEBUFFER, self.g_buffer);
            self.gl.BindFramebuffer(GL::DRAW_FRAMEBUFFER, 0);
            self.gl.BlitFramebuffer(0, 0,
                self.framebuffer_size.0, self.framebuffer_size.1,
                0, 0,
                self.framebuffer_size.0, self.framebuffer_size.1,
                GL::DEPTH_BUFFER_BIT, GL::NEAREST);
            self.gl.BindFramebuffer(GL::FRAMEBUFFER, 0);
        }
    }

    pub fn render_lighting(&self, camera: &Camera, shader: &Shader, lights: &Vec<(&BlockLight, &BlockBuffer)>) {
        shader.bind();
        shader.mat_4("projection", camera.projection());
        shader.mat_4("view", camera.view());
        for (light, buffer) in lights.iter() {
            let block = block_database::get().get_block(light.block_type);
            shader.vec3("lightColor", &light.color);
            let block_scale = block.model_scale.unwrap_or(1.0);
            buffer.draw(shader, &scale(
                &block.get_model(&light.position, 0.0, 0.0),
                vector!(block_scale, block_scale, block_scale)
            ));
        }
    }
}