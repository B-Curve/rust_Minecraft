use GL::Gl;
use render::deferred_buffer::DeferredBuffer;
use shader::{Shader, Type as ShaderType};
use std::collections::HashMap;
use world::block::block::Block;
use std::rc::Rc;
use world::block::block_buffer::BlockBuffer;
use world::block::block_light::BlockLight;
use util::math::Vec3;
use camera::Camera;
use world::block::block_texture::BlockTexture;
use world::block::block_type::BlockType;
use world::block::block_database;

pub struct Lighting {
    buffer: DeferredBuffer,
    item_shader: Shader,
    lights: HashMap<(i32, i32, i32), (BlockLight, BlockBuffer)>,
    gl: Gl
}

impl Lighting {
    pub fn new(gl: &Gl) -> Lighting {
        let buffer = DeferredBuffer::new(gl);
        let lighting_shader = Shader::new(gl, ShaderType::Deferred, false).unwrap();
        let item_shader = Shader::new(gl, ShaderType::Torch, false).unwrap();


        buffer.shader().bind();
        buffer.shader().int("gPosition", 0);
        buffer.shader().int("gNormal", 1);
        buffer.shader().int("gAlbedoSpec", 2);

        Lighting { buffer, gl: gl.clone(), item_shader, lights: HashMap::new() }
    }

    pub fn get_light_vec(&self) -> Vec<&BlockLight> {
        self.lights.iter().map(|(_, (l, b))| l).collect::<Vec<&BlockLight>>()
    }

    pub fn add_light(&mut self, block_type: BlockType, position: Vec3) {
        let (x, y, z) = (position.x as i32, position.y as i32, position.z as i32);
        let block = block_database::get().get_block(block_type);
        let block_light = block.get_light(&position);
        let block_buffer = BlockBuffer::new(&self.gl, block);
        self.lights.insert((x, y, z), (block_light, block_buffer));
    }

    pub fn bind_framebuffer(&self) {
        self.buffer.bind_framebuffer();
    }

    pub fn unbind_framebuffer(&self) {
        self.buffer.unbind_framebuffer();
    }

    pub fn apply_lighting(&self, position: &Vec3) {
        self.buffer.apply_lighting(
            position,
            &self.get_light_vec());
    }

    pub fn copy_depth_buffer(&self) {
        self.buffer.copy_depth_buffer();
    }

    pub fn render_lighting(&self, camera: &Camera) {
        self.buffer.render_lighting(
            camera,
            &self.item_shader,
            &self.lights.iter().map(|(_, (l, b))| (l, b)).collect::<Vec<(&BlockLight, &BlockBuffer)>>());
    }
}