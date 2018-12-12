use GL;
use GL::Gl;
use stb_image::stb_image::bindgen::{stbi_load, stbi_image_free};
use std::os::raw::{c_int, c_void};
use std::ffi::CString;

const BLOCK_MAP: &'static str = "./assets/textures/block_map.png";

pub struct BlockTexture {
    id: u32,
    gl: Gl
}

impl BlockTexture {
    pub fn new(gl: &Gl) -> BlockTexture {
        let mut id = 0;
        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(GL::TEXTURE_2D, id);

            let file = CString::new(BLOCK_MAP).unwrap();

            let (mut width, mut height, mut nr_channels): (c_int, c_int, c_int) = (0, 0, 0);
            let data = stbi_load(file.as_ptr(), &mut width, &mut height, &mut nr_channels, 0);
            if !data.is_null() {
                let format = match nr_channels {
                    1 => GL::RED as i32,
                    3 => GL::RGB as i32,
                    4 => GL::RGBA as i32,
                    _ => GL::RGB as i32
                };
                gl.TexImage2D(GL::TEXTURE_2D, 0, format, width, height, 0, format as u32, GL::UNSIGNED_BYTE, data as *const ::std::ffi::c_void);
                gl.GenerateMipmap(GL::TEXTURE_2D);
                gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
                gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
                gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
                gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
            } else {
                panic!("Failed to load texture from path: {}", BLOCK_MAP);
            }

            stbi_image_free(data as *mut c_void);
        }

        BlockTexture {id, gl: gl.clone()}
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            self.gl.ActiveTexture(GL::TEXTURE0 + unit);
            self.gl.BindTexture(GL::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for BlockTexture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &mut self.id);
        }
    }
}