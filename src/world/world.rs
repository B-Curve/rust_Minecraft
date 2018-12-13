use GL::Gl;

use threadpool::ThreadPool;
use std::sync::{Arc, Mutex, MutexGuard, mpsc};
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
    player_spawn: Arc<Mutex<Option<Vec3>>>,
    thread_pool: ThreadPool
}

impl World {
    pub fn new(gl: &Gl) -> World {
        let mut chunk_queue: Arc<Mutex<Vec<Chunk>>> = Arc::new(Mutex::new(Vec::new()));
        let block_texture = BlockTexture::new(gl);

        World { chunk_queue, active_chunks: HashMap::new(),
            gl: gl.clone(), block_texture,
            player_spawn: Arc::new(Mutex::new(None)),
            thread_pool: ThreadPool::new(4)
        }
    }

    pub fn initialize_chunks(&mut self) {
        let c = World::chunk_coordinates(&Vec3::new(0.0, 0.0, 0.0));
        let (xx, zz, mx, mz) = (
            c.0 - 2,
            c.1 - 2,
            c.0 + 2,
            c.1 + 2
        );

        let (tx, rx) = mpsc::channel();

        for x in xx..mx {
            for z in zz..mz {
                let tx = tx.clone();
                let mut queue = self.chunk_queue.clone();
                let mut spawn = self.player_spawn.clone();
                self.thread_pool.execute(move|| {
                    let chunk = Chunk::new(Vec3i::new(x, 0, z));
                    if chunk.index() == (0, 0) {
                        let spawn_y = chunk.get_highest_block((0, 0));
                        let mut state = spawn.lock().unwrap();
                        ::std::mem::replace(&mut *state, Some(Vec3::new(0.0, (spawn_y + 1) as f32, 0.0)));
                    }
                    tx.send(queue.lock().unwrap().push(chunk));
                });
            }
        }
    }

    pub fn get_player_spawn(&self) -> MutexGuard<Option<Vec3>> {
        self.player_spawn.lock().unwrap()
    }

    pub fn build_chunks(&mut self, position: &Vec3) {
        let c = World::chunk_coordinates(position);
        let (xx, zz, mx, mz) = (
            c.0 - RENDER_DISTANCE,
            c.1 - RENDER_DISTANCE,
            c.0 + RENDER_DISTANCE,
            c.1 + RENDER_DISTANCE
        );

        let pool = ThreadPool::new(4);
        let (tx, rx) = mpsc::channel();

        for x in xx..mx {
            for z in zz..mz {
                if self.active_chunks.contains_key(&(x, z)) { continue; }
                let tx = tx.clone();
                let mut queue = self.chunk_queue.clone();
                self.thread_pool.execute(move|| {
                    let chunk = Chunk::new(Vec3i::new(x, 0, z));
                    tx.send(queue.lock().unwrap().push(chunk));
                });
            }
        }
    }

    pub fn take_chunk_from_queue(&mut self) {
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