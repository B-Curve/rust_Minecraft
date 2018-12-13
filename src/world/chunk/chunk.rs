use math::{vec3, Vec3, Vec3i, Mat4, One, Ext::translate};
use std::collections::HashMap;
use world::chunk::chunk_mesh::ChunkMesh;
use world::block::block_type::BlockType;
use world::constants::CHUNK_SIZE;
use world::constants::CHUNK_HEIGHT;
use util::vertex::Vertex;

pub struct Chunk {
    mesh: ChunkMesh,
    world_position: Vec3i,
    index_position: Vec3i,
    block_map: HashMap<(i32, i32, i32), BlockType>
}

impl Chunk {
    pub fn new(index_position: Vec3i) -> Chunk {
        let mut block_map: HashMap<(i32, i32, i32), BlockType> = HashMap::new();

        for x in 0..CHUNK_SIZE {
        for z in 0..CHUNK_SIZE {
            let mut highest_dirt = -1;
            for y in -1..CHUNK_HEIGHT {
                if y == -1 {
                    block_map.insert((x, y, z), BlockType::Bedrock);
                    continue;
                }
                let block_type = BlockType::noise_natural(Vec3i::new(
                    x + (index_position.x * CHUNK_SIZE),
                    y,
                    z + (index_position.z * CHUNK_SIZE)
                ));
                if block_type == BlockType::Dirt { highest_dirt = y; }
                block_map.insert((x,y,z), block_type);
            }
            if highest_dirt > 0 {
                block_map.entry((x, highest_dirt, z)).and_modify(|b| *b = BlockType::Grass);
            }
        }}

        Chunk {
            mesh: ChunkMesh::new(&block_map),
            world_position: Vec3i::new(index_position.x * CHUNK_SIZE, 0, index_position.z * CHUNK_SIZE),
            index_position,
            block_map
        }
    }

    pub fn get_highest_block(&self, pos_xz: (i32, i32)) -> i32 {
        let (x, z) = pos_xz;
        let mut highest_block = 0;
        for y in 0..CHUNK_HEIGHT {
            if let Some(block_type) = self.block_map.get(&(x, y, z)) {
                if block_type == &BlockType::Air { continue; }
                highest_block = y;
            }
        }
        highest_block
    }

    pub fn index(&self) -> (i32, i32) {
        (self.index_position.x, self.index_position.z)
    }

    pub fn verts(&self) -> &Vec<Vertex> {
        self.mesh.verts()
    }

    pub fn inds(&self) -> &Vec<u32> {
        self.mesh.inds()
    }

    pub fn model(&self) -> Mat4 {
        translate(&Mat4::one(), vec3(
            self.world_position.x as f32,
            self.world_position.y as f32,
            self.world_position.z as f32)
        )
    }
}