use GL::Gl;

use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use util::math::{Vec3, Vec2i, Vec3i};
use world::constants::CHUNK_SIZE;
use world::constants::RENDER_DISTANCE;
use world::chunk::chunk::Chunk;
use world::block::block_texture::BlockTexture;
use shader::Shader;
use camera::Camera;
use world::chunk::chunk_buffer::ChunkBuffer;

pub struct World {
    chunk_queue: Arc<Mutex<Vec<Chunk>>>,
    active_chunks: HashMap<(i32, i32), (Chunk, ChunkBuffer)>,
    gl: Gl,
    block_texture: BlockTexture,
}

impl World {
    pub fn new(gl: &Gl, position: &Vec3) -> World {
        let c = World::chunk_coordinates(position);
        let (xx, zz, mx, mz) = (c.0 - RENDER_DISTANCE, c.1 - RENDER_DISTANCE, c.0 + RENDER_DISTANCE, c.1 + RENDER_DISTANCE);
        let mut chunk_queue: Arc<Mutex<Vec<Chunk>>> = Arc::new(Mutex::new(Vec::new()));
        let block_texture = BlockTexture::new(gl);

        let mut queue = chunk_queue.clone();
        let mut t = thread::spawn(move || {
            for x in xx..mx {
                for z in zz..mz {
                    queue.lock().unwrap().push(Chunk::new(Vec3i::new(x, 0, z)));
                }
            }
        });
        t.join();

        World { chunk_queue, active_chunks: HashMap::new(), gl: gl.clone(), block_texture }
    }

    pub fn build_chunk_from_queue(&mut self) {
        let length = self.chunk_queue.lock().unwrap().len();
        if length > 0 {
            let chunk = self.chunk_queue.lock().unwrap().swap_remove(0);
            let buffer = ChunkBuffer::new(&self.gl, chunk.verts(), chunk.inds());
            self.active_chunks.insert(chunk.index(), (
                chunk, buffer
            ));
        }
    }

    pub fn render(&self, block_shader: &Shader, camera: &Camera) {
        self.bind_block_texture(0);
        block_shader.bind();
        block_shader.mat_4("view", camera.view());
        block_shader.mat_4("projection", camera.projection());
        block_shader.int("tex", 0);
        for (_, (chunk, buffer)) in self.active_chunks.iter() {
            buffer.draw(block_shader, &chunk.model());
        }
    }

    pub fn bind_block_texture(&self, loc: u32) {
        self.block_texture.bind(loc);
    }

    fn chunk_coordinates(position: &Vec3) -> (i32, i32) {
        (position.x as i32 / CHUNK_SIZE, position.z as i32 / CHUNK_SIZE)
    }
}