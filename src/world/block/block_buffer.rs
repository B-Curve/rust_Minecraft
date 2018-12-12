use GL;
use GL::Gl;
use world::block::block::Block;
use shader::Shader;
use util::vertex::Vertex;
use util::math::{Mat4, Vec3i};
use world::block::block_database;
use world::block::block_type::BlockType;
use std::mem::size_of;

pub struct BlockBuffer {
    gl: Gl,
    vao: u32,
    vbo: u32,
    ibo: u32,
    block_type: BlockType
}

impl BlockBuffer {
    pub fn new(gl: &Gl, block: &Block) -> BlockBuffer {
        let (mut vbo, mut vao, mut ibo) = (0, 0, 0);

        let funcs = vec![
            block.build_front_face(&Vec3i::new(0, 0, 0), 0),
            block.build_back_face(&Vec3i::new(0, 0, 0), 4),
            block.build_left_face(&Vec3i::new(0, 0, 0), 8),
            block.build_right_face(&Vec3i::new(0, 0, 0), 12),
            block.build_top_face(&Vec3i::new(0, 0, 0), 16),
            block.build_bottom_face(&Vec3i::new(0, 0, 0), 20)
        ];
        let mut verts = Vec::new();
        let mut inds = Vec::new();

        for (mut v, mut i) in funcs {
            verts.append(&mut v);
            inds.append(&mut i);
        }

        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);
            gl.GenBuffers(1, &mut ibo);
            gl.BindVertexArray(vao);

            gl.BindBuffer(GL::ARRAY_BUFFER, vbo);
            gl.BufferData(GL::ARRAY_BUFFER, (verts.len() * size_of::<Vertex>()) as isize, verts.as_ptr() as *const ::std::ffi::c_void, GL::STATIC_DRAW);

            gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, ibo);
            gl.BufferData(GL::ELEMENT_ARRAY_BUFFER, (inds.len() * size_of::<u32>()) as isize, inds.as_ptr() as *const ::std::ffi::c_void, GL::STATIC_DRAW);

            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(0, 3, GL::FLOAT, GL::FALSE, size_of::<Vertex>() as i32, ::std::ptr::null());

            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(1, 2, GL::FLOAT, GL::FALSE, size_of::<Vertex>() as i32, (3 * size_of::<f32>()) as *const ::std::ffi::c_void);

            gl.EnableVertexAttribArray(2);
            gl.VertexAttribPointer(2, 3, GL::FLOAT, GL::FALSE, size_of::<Vertex>() as i32, (5 * size_of::<f32>()) as *const ::std::ffi::c_void);
        }

        BlockBuffer { gl: gl.clone(), vao, vbo, ibo, block_type: block.m_type }
    }

    pub fn draw(&self, shader: &Shader, model: &Mat4) {
        unsafe {
            shader.mat_4("model", model);
            self.gl.BindVertexArray(self.vao);
            self.gl.DrawElements(GL::TRIANGLES, 36, GL::UNSIGNED_INT, ::std::ptr::null());
            self.gl.BindVertexArray(0);
        }
    }
}