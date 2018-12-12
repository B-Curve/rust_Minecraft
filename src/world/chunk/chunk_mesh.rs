use util::vertex::Vertex;
use world::block::block::Block;
use std::collections::HashMap;
use math::{Vec3i, Mat4, vec3, Ext::{translate}, One};
use world::block::block_type::BlockType;
use world::block::block_database;

pub struct ChunkMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>
}

impl ChunkMesh {
    pub fn new(block_positions: &HashMap<(i32, i32, i32), BlockType>) -> ChunkMesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut index_stride = 4;
        let mut current_stride = 0;

        for ((x, y, z), block_type) in block_positions.iter() {
            let (x, y, z) = (x.clone(), y.clone(), z.clone());

            let block = block_database::get().get_block(block_type.clone());
            if block.m_type == BlockType::Air { continue; }

            let right = block_database::get().unwrap_block(block_positions.get(&(x + 1, y, z)));
            let left = block_database::get().unwrap_block(block_positions.get(&(x - 1, y, z)));
            let top = block_database::get().unwrap_block(block_positions.get(&(x, y + 1, z)));
            let bottom = block_database::get().unwrap_block(block_positions.get(&(x, y - 1, z)));
            let front = block_database::get().unwrap_block(block_positions.get(&(x, y, z - 1)));
            let back = block_database::get().unwrap_block(block_positions.get(&(x, y, z + 1)));

            if block.has_sub1_scale() || !right.opaque {
                let (mut verts, mut inds) = block.build_right_face(
                    &Vec3i::new(x,y,z), current_stride);
                vertices.append(&mut verts);
                indices.append(&mut inds);
                current_stride += index_stride;
            }
            if block.has_sub1_scale() || !left.opaque {
                let (mut verts, mut inds) = block.build_left_face(
                    &Vec3i::new(x,y,z), current_stride);
                vertices.append(&mut verts);
                indices.append(&mut inds);
                current_stride += index_stride;
            }
            if block.has_sub1_scale() || !top.opaque {
                let (mut verts, mut inds) = block.build_top_face(
                    &Vec3i::new(x,y,z), current_stride);
                vertices.append(&mut verts);
                indices.append(&mut inds);
                current_stride += index_stride;
            }
            if block.has_sub1_scale() || !bottom.opaque {
                let (mut verts, mut inds) = block.build_bottom_face(
                    &Vec3i::new(x,y,z), current_stride);
                vertices.append(&mut verts);
                indices.append(&mut inds);
                current_stride += index_stride;
            }
            if block.has_sub1_scale() || !front.opaque {
                let (mut verts, mut inds) = block.build_front_face(
                    &Vec3i::new(x,y,z), current_stride);
                vertices.append(&mut verts);
                indices.append(&mut inds);
                current_stride += index_stride;
            }
            if block.has_sub1_scale() || !back.opaque {
                let (mut verts, mut inds) = block.build_back_face(
                    &Vec3i::new(x,y,z), current_stride);
                vertices.append(&mut verts);
                indices.append(&mut inds);
                current_stride += index_stride;
            }
        }

        ChunkMesh { vertices, indices }
    }

    pub fn verts(&self) -> &Vec<Vertex> { &self.vertices }
    pub fn inds(&self) -> &Vec<u32> { &self.indices }
}