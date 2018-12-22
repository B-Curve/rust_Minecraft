use GL;
use GL::Gl;
use std::mem::size_of;
use std::ffi::c_void;
use stb_image::stb_image::bindgen::{stbi_load, stbi_image_free};
use std::os::raw::{c_int, c_void as void};
use std::ffi::CString;
use shader::{Shader, Type as ShaderType};

pub struct SkyBox {
    vao: u32,
    vbo: u32,
    texture_id: u32,
    shader: Shader,
    gl: Gl
}

impl SkyBox {
    pub fn new(gl: &Gl) -> SkyBox {
        let (mut vao, mut vbo, mut texture_id) = (0, 0, 0);
        unsafe {
            let faces = vec![
                CString::new("./assets/textures/sky/map_front.tga").unwrap(),
                CString::new("./assets/textures/sky/map_back.tga").unwrap(),
                CString::new("./assets/textures/sky/map_top.tga").unwrap(),
                CString::new("./assets/textures/sky/map_bottom.tga").unwrap(),
                CString::new("./assets/textures/sky/map_right.tga").unwrap(),
                CString::new("./assets/textures/sky/map_left.tga").unwrap()
            ];
            gl.GenTextures(1, &mut texture_id);
            gl.BindTexture(GL::TEXTURE_CUBE_MAP, texture_id);

            let (mut width, mut height, mut channels) = (0, 0, 0);
            for (i, face) in faces.iter().enumerate() {
                let data = stbi_load(face.as_ptr(), &mut width, &mut height, &mut channels, 0);
                if !data.is_null() {
                    gl.TexImage2D(GL::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32, 0, GL::RGB as i32, width, height, 0, GL::RGB, GL::UNSIGNED_BYTE, data as *const c_void);
                } else {
                    panic!("Failed to load cubemap: {}", face.to_str().unwrap());
                }
                stbi_image_free(data as *mut void);
            }
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_WRAP_R, GL::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
            gl.TexParameteri(GL::TEXTURE_CUBE_MAP, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);
            gl.BindVertexArray(vao);
            gl.BindBuffer(GL::ARRAY_BUFFER, vbo);
            gl.BufferData(GL::ARRAY_BUFFER, (size_of::<f32>() * VERTICES.len()) as isize, VERTICES.as_ptr() as *const c_void, GL::STATIC_DRAW);
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(0, 3, GL::FLOAT, GL::FALSE, (3 * size_of::<f32>()) as i32, ::std::ptr::null());
        }

        SkyBox { vao, vbo, texture_id,
            shader: Shader::new(gl, ShaderType::CubeMap, false).unwrap(),
            gl: gl.clone() }
    }

    pub fn draw(&self, view: &::math::Mat4, projection: &::math::Mat4) {
        use math::{Mat4, vec4};
        unsafe {
            self.gl.DepthFunc(GL::LEQUAL);
            self.shader.bind();
            self.shader.mat_4("view", &Mat4::new(
                vec4(view.c0.x, view.c0.y, view.c0.z, 0.0),
                vec4(view.c1.x, view.c1.y, view.c1.z, 0.0),
                vec4(view.c2.x, view.c2.y, view.c2.z, 0.0),
                vec4(0.0, 0.0, 0.0, 0.0)
            ));
            self.shader.mat_4("projection", projection);
            self.gl.BindVertexArray(self.vao);
            self.gl.ActiveTexture(GL::TEXTURE0);
            self.gl.BindTexture(GL::TEXTURE_CUBE_MAP, self.texture_id);
            self.gl.DrawArrays(GL::TRIANGLES, 0, 36);
            self.gl.BindVertexArray(0);
            self.gl.DepthFunc(GL::LESS);
        }
    }
}

impl Drop for SkyBox {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &mut self.texture_id);
            self.gl.DeleteVertexArrays(1, &mut self.vao);
            self.gl.DeleteBuffers(1, &mut self.vbo);
        }
    }
}

const VERTICES: [f32; 108] = [
    -1.0,  1.0, -1.0,
    -1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,
    1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,

    -1.0, -1.0,  1.0,
    -1.0, -1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0, -1.0,
    -1.0,  1.0,  1.0,
    -1.0, -1.0,  1.0,

    1.0, -1.0, -1.0,
    1.0, -1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0, -1.0,
    1.0, -1.0, -1.0,

    -1.0, -1.0,  1.0,
    -1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    1.0, -1.0,  1.0,
    -1.0, -1.0,  1.0,

    -1.0,  1.0, -1.0,
    1.0,  1.0, -1.0,
    1.0,  1.0,  1.0,
    1.0,  1.0,  1.0,
    -1.0,  1.0,  1.0,
    -1.0,  1.0, -1.0,

    -1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
    1.0, -1.0, -1.0,
    1.0, -1.0, -1.0,
    -1.0, -1.0,  1.0,
    1.0, -1.0,  1.0
];