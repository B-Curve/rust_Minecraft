use GL;
use GL::Gl;
use freetype;
use math::{vec2, Vec2i, Vec3, ortho};
use std::collections::HashMap;
use shader::{Shader, Type as ShaderType};
use util::resources;

struct Character {
    texture_id: u32,
    size: Vec2i,
    bearing: Vec2i,
    advance: i32,
    gl: Gl
}

impl Drop for Character {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &mut self.texture_id);
        }
    }
}

pub struct Text {
    characters: HashMap<char, Character>,
    vao: u32,
    vbo: u32,
    gl: Gl,
    shader: Shader
}

impl Text {
    pub fn from_font(gl: &Gl, file: &str) -> Text {
        unsafe {
            let mut shader = Shader::new(gl, ShaderType::Text, false).unwrap();
            let mut characters = HashMap::new();
            let (mut vao, mut vbo) = (0, 0);

            shader.bind();
            shader.mat_4("projection", &ortho(0.0, 1920.0, 0.0, 1080.0, -1.0, 100.0));

            let face = resources::load_font(file).unwrap();
            face.set_pixel_sizes(0, 48);

            gl.PixelStorei(GL::UNPACK_ALIGNMENT, 1);

            let ref g = face.glyph();
            let ref bmp = face.glyph().bitmap();

            for c in 0..128u8 {
                face.load_char(c as usize, freetype::face::LoadFlag::RENDER).unwrap();
                let mut texture: u32 = 0;
                gl.GenTextures(1, &mut texture);
                gl.BindTexture(GL::TEXTURE_2D, texture);
                gl.TexImage2D(GL::TEXTURE_2D, 0, GL::RED as i32, bmp.width(), bmp.rows(),
                              0, GL::RED, GL::UNSIGNED_BYTE, bmp.buffer().as_ptr() as *const ::std::ffi::c_void);
                gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
                gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);
                gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR as i32);
                gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR as i32);

                let ch = Character {
                    texture_id: texture,
                    size: Vec2i::new(bmp.width(), bmp.rows()),
                    bearing: Vec2i::new(g.bitmap_left(), g.bitmap_top()),
                    advance: g.advance().x,
                    gl: gl.clone()
                };
                characters.insert(c as char, ch);
            }
            gl.BindTexture(GL::TEXTURE_2D, 0);

            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);
            gl.BindVertexArray(vao);
            gl.BindBuffer(GL::ARRAY_BUFFER, vbo);
            gl.BufferData(GL::ARRAY_BUFFER, (::std::mem::size_of::<f32>() * 6 * 4) as isize, ::std::ptr::null(), GL::DYNAMIC_DRAW);
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(0, 4, GL::FLOAT, GL::FALSE, (4 * ::std::mem::size_of::<f32>()) as i32, ::std::ptr::null() as *const ::std::ffi::c_void);
            gl.BindBuffer(GL::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);

            Text { characters, gl: gl.clone(), vao, vbo, shader }
        }
    }

    pub fn render(&mut self, text: &str, x: f32, y: f32, s: f32, color: &Vec3) {
        unsafe {
            self.gl.Enable(GL::BLEND);
            self.shader.bind();
            self.shader.vec3("textColor", color);
            self.gl.ActiveTexture(GL::TEXTURE0);
            self.gl.BindVertexArray(self.vao);

            let mut xx = x;
            for c in text.chars() {
                let ch = self.characters.get(&c).unwrap();

                let x_pos = xx + ch.bearing.x as f32 * s;
                let y_pos = y - (ch.size.y - ch.bearing.y) as f32 * s;
                let w = ch.size.x as f32 * s;
                let h = ch.size.y as f32 * s;
                let verts: [[f32; 4]; 6] = [
                    [x_pos, y_pos + h, 0.0, 0.0],
                    [x_pos, y_pos, 0.0, 1.0],
                    [x_pos + w, y_pos, 1.0, 1.0],
                    [x_pos, y_pos + h, 0.0, 0.0],
                    [x_pos + w, y_pos, 1.0, 1.0],
                    [x_pos + w, y_pos + h, 1.0, 0.0]
                ];

                self.gl.BindTexture(GL::TEXTURE_2D, ch.texture_id);
                self.gl.BindBuffer(GL::ARRAY_BUFFER, self.vbo);
                self.gl.BufferSubData(GL::ARRAY_BUFFER, 0, ::std::mem::size_of_val(&verts) as isize, verts.as_ptr() as *const ::std::ffi::c_void);
                self.gl.BindBuffer(GL::ARRAY_BUFFER, 0);
                self.gl.DrawArrays(GL::TRIANGLES, 0, 6);
                xx += (ch.advance >> 6) as f32 * s;
            }
            self.gl.BindVertexArray(0);
            self.gl.BindTexture(GL::TEXTURE_2D, 0);
            self.gl.Disable(GL::BLEND);
        }
    }
}

impl Drop for Text {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &mut self.vbo);
            self.gl.DeleteVertexArrays(1, &mut self.vao);
        }
    }
}