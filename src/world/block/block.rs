use serde;
use serde_json;
use serde_json::Error;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use math::{Vec2, vec2, Vec3, vec3, Vec3i, Mat4, One, Ext::{scale, rotate, translate}};
use world::block::block_type::{BlockType, UvCoords};
use util::vertex::Vertex;
use world::block::block_light::BlockLight;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockList {
    pub blocks: Vec<Block>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub m_type: BlockType,
    pub id: u8,
    pub opaque: bool,
    pub has_texture: bool,
    pub texture_top: Option<Vec<f32>>,
    pub texture_bottom: Option<Vec<f32>>,
    pub texture_front: Option<Vec<f32>>,
    pub texture_back: Option<Vec<f32>>,
    pub texture_left: Option<Vec<f32>>,
    pub texture_right: Option<Vec<f32>>,
    pub scale_x: Option<f32>,
    pub scale_y: Option<f32>,
    pub light_emission: f32,
    pub light_color: Option<Vec<f32>>
}

impl Block {
    pub fn new(filename: &str) -> Block {
        let file = File::open(filename).expect(format!("Failed to open file: {}", filename).as_str());
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect(format!("Failed to read contents of file: {}", filename).as_str());
        serde_json::from_str::<Block>(&contents)
            .expect(format!("Failed to parse contents to string for file: {}", filename).as_str())
    }

    pub fn get_light(&self, position: &Vec3) -> BlockLight {
        if let Some(c) = &self.light_color {
            BlockLight {
                color: vec3(c[0] / 255.0, c[1] / 255.0, c[2] / 255.0),
                strength: self.light_emission,
                position: position.clone(),
                block_type: self.m_type
            }
        } else {
            Default::default()
        }
    }

    pub fn has_sub1_scale(&self) -> bool {
        self.scale_x.unwrap_or(1.0) < 1.0
        || self.scale_y.unwrap_or(1.0) < 1.0
    }

    pub fn scale(&self, model: &Mat4) -> Mat4 {
        scale(&model, vec3(
            self.scale_x.unwrap_or(1.0),
            self.scale_y.unwrap_or(1.0),
            self.scale_x.unwrap_or(1.0)
        ))
    }

    pub fn get_model(&self, translation: &Vec3, rot_x: f32, rot_y: f32) -> Mat4 {
        let mut m = translate(&Mat4::one(), translation.clone());
        m = self.scale(&m);
        m = rotate(&m, rot_x, vec3(1.0, 0.0, 0.0));
        m = rotate(&m, rot_y, vec3(0.0, 1.0, 0.0));
        m
    }

    fn get_uv(tex_coords: &Option<Vec<f32>>, scale_x: f32, scale_y: f32) -> UvCoords {
        match tex_coords {
            Some(coords) => BlockType::tex_coords(vec2(coords[0], coords[1]), scale_x, scale_y),
            None => Default::default()
        }
    }

    fn vi_v(&self, v: &Vec3i) -> Vec3 {
        vec3(
            v.x as f32,
            v.y as f32,
            v.z as f32
        )
    }

    fn get_scale(&self, translate_from_scale: bool) -> Vec3 {
        match translate_from_scale {
            true => vec3(self.scale_x.unwrap_or(1.0), self.scale_y.unwrap_or(1.0), self.scale_x.unwrap_or(1.0)),
            false => vec3(1.0,1.0,1.0)
        }
    }

    pub fn build_back_face(&self, translation: &Vec3i, off: u32, translate_from_scale: bool) -> (Vec<Vertex>, Vec<u32>) {
        let uv = Block::get_uv(&self.texture_back, self.scale_x.unwrap_or(1.0), self.scale_y.unwrap_or(1.0));
        let t = self.vi_v(translation);
        let s = self.get_scale(translate_from_scale);
        (vec![
            Vertex { position: vec3(-0.5, -0.5, 0.5) * s + t, uv: uv.a, normal: vec3(0.0, 0.0, 1.0) },
            Vertex { position: vec3( 0.5, -0.5, 0.5) * s + t, uv: uv.b, normal: vec3(0.0, 0.0, 1.0) },
            Vertex { position: vec3( 0.5,  0.5, 0.5) * s + t, uv: uv.c, normal: vec3(0.0, 0.0, 1.0) },
            Vertex { position: vec3(-0.5,  0.5, 0.5) * s + t, uv: uv.d, normal: vec3(0.0, 0.0, 1.0) },
        ], vec![off, off + 1, off + 2, off + 2, off + 3, off])
    }

    pub fn build_front_face(&self, translation: &Vec3i, off: u32, translate_from_scale: bool) -> (Vec<Vertex>, Vec<u32>) {
        let uv = Block::get_uv(&self.texture_front, self.scale_x.unwrap_or(1.0), self.scale_y.unwrap_or(1.0));
        let t = self.vi_v(translation);
        let s = self.get_scale(translate_from_scale);
        (vec![
            Vertex { position: vec3(-0.5, -0.5, -0.5) * s + t, uv: uv.a, normal: vec3(0.0, 0.0, -1.0) },
            Vertex { position: vec3( 0.5, -0.5, -0.5) * s + t, uv: uv.b, normal: vec3(0.0, 0.0, -1.0) },
            Vertex { position: vec3( 0.5,  0.5, -0.5) * s + t, uv: uv.c, normal: vec3(0.0, 0.0, -1.0) },
            Vertex { position: vec3(-0.5,  0.5, -0.5) * s + t, uv: uv.d, normal: vec3(0.0, 0.0, -1.0) },
        ], vec![off, off + 2, off + 1, off + 2, off, off + 3])
    }

    pub fn build_left_face(&self, translation: &Vec3i, off: u32, translate_from_scale: bool) -> (Vec<Vertex>, Vec<u32>) {
        let uv = Block::get_uv(&self.texture_left, self.scale_x.unwrap_or(1.0), self.scale_y.unwrap_or(1.0));
        let t = self.vi_v(translation);
        let s = self.get_scale(translate_from_scale);
        (vec![
            Vertex { position: vec3(-0.5, -0.5, 0.5) * s + t, uv: uv.a, normal: vec3(-1.0, 0.0, 0.0) },
            Vertex { position: vec3( -0.5, -0.5, -0.5) * s + t, uv: uv.b, normal: vec3(-1.0, 0.0, 0.0) },
            Vertex { position: vec3( -0.5,  0.5, -0.5) * s + t, uv: uv.c, normal: vec3(-1.0, 0.0, 0.0) },
            Vertex { position: vec3(-0.5,  0.5, 0.5) * s + t, uv: uv.d, normal: vec3(-1.0, 0.0, 0.0) },
        ], vec![off + 2, off, off + 3, off, off + 2, off + 1])
    }

    pub fn build_right_face(&self, translation: &Vec3i, off: u32, translate_from_scale: bool) -> (Vec<Vertex>, Vec<u32>) {
        let uv = Block::get_uv(&self.texture_right, self.scale_x.unwrap_or(1.0), self.scale_y.unwrap_or(1.0));
        let t = self.vi_v(translation);
        let s = self.get_scale(translate_from_scale);
        (vec![
            Vertex { position: vec3(0.5, -0.5, -0.5) * s + t, uv: uv.a, normal: vec3(1.0, 0.0, 0.0) },
            Vertex { position: vec3( 0.5, -0.5, 0.5) * s + t, uv: uv.b, normal: vec3(1.0, 0.0, 0.0) },
            Vertex { position: vec3( 0.5,  0.5, 0.5) * s + t, uv: uv.c, normal: vec3(1.0, 0.0, 0.0) },
            Vertex { position: vec3(0.5,  0.5, -0.5) * s + t, uv: uv.d, normal: vec3(1.0, 0.0, 0.0) },
        ], vec![off + 3, off + 2, off + 1, off + 1, off, off + 3])
    }

    pub fn build_top_face(&self, translation: &Vec3i, off: u32, translate_from_scale: bool) -> (Vec<Vertex>, Vec<u32>) {
        let uv = Block::get_uv(&self.texture_top, self.scale_x.unwrap_or(1.0), self.scale_y.unwrap_or(1.0));
        let t = self.vi_v(translation);
        let s = self.get_scale(translate_from_scale);
        (vec![
            Vertex { position: vec3(-0.5, 0.5, -0.5) * s + t, uv: uv.a, normal: vec3(0.0, 1.0, 0.0) },
            Vertex { position: vec3( 0.5, 0.5, -0.5) * s + t, uv: uv.b, normal: vec3(0.0, 1.0, 0.0) },
            Vertex { position: vec3( 0.5,  0.5, 0.5) * s + t, uv: uv.c, normal: vec3(0.0, 1.0, 0.0) },
            Vertex { position: vec3(-0.5,  0.5, 0.5) * s + t, uv: uv.d, normal: vec3(0.0, 1.0, 0.0) },
        ], vec![off + 3, off + 2, off + 1, off + 1, off, off + 3])
    }

    pub fn build_bottom_face(&self, translation: &Vec3i, off: u32, translate_from_scale: bool) -> (Vec<Vertex>, Vec<u32>) {
        let uv = Block::get_uv(&self.texture_bottom, self.scale_x.unwrap_or(1.0), self.scale_y.unwrap_or(1.0));
        let t = self.vi_v(translation);
        let s = self.get_scale(translate_from_scale);
        (vec![
            Vertex { position: vec3(-0.5, -0.5, -0.5) * s + t, uv: uv.a, normal: vec3(0.0, -1.0, 0.0) },
            Vertex { position: vec3( 0.5, -0.5, -0.5) * s + t, uv: uv.b, normal: vec3(0.0, -1.0, 0.0) },
            Vertex { position: vec3( 0.5,  -0.5, 0.5) * s + t, uv: uv.c, normal: vec3(0.0, -1.0, 0.0) },
            Vertex { position: vec3(-0.5,  -0.5, 0.5) * s + t, uv: uv.d, normal: vec3(0.0, -1.0, 0.0) },
        ], vec![off, off + 1, off + 2, off + 2, off + 3, off])
    }
}