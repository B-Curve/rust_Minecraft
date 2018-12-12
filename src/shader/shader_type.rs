use std::ffi::CString;
use std::fs::File;
use std::io::Read;

pub enum Type {
    Block,
    Text,
    CubeMap,
    Torch,
    Deferred,
    ShadowMap
}

pub struct ShaderString {
    pub vertex: CString,
    pub fragment: CString,
    pub geometry: Option<CString>
}

impl Type {
    pub fn as_str(&self) -> &str {
        use self::Type::*;
        match *self {
            Block => "block",
            Text => "text",
            CubeMap => "cube_map",
            Torch => "torch",
            Deferred => "deferred",
            ShadowMap => "shadow_map"
        }
    }

    pub fn file_bytes_as_cstr(&self, include_geometry: bool) -> Result<ShaderString, ::std::io::Error> {
        use self::Type::*;
        let p: &str = &("./assets/shaders/".to_string() + self.as_str());
        let vertex = load_specific_shader(p, ".vert")?;
        let fragment = load_specific_shader(p, ".frag")?;

        Ok(ShaderString {
            vertex,
            fragment,
            geometry: match include_geometry {
                true => Some(load_specific_shader(p, ".geom")?),
                false => None
            }
        })
    }
}

fn load_specific_shader(filename: &str, extension: &str) -> Result<CString, ::std::io::Error> {
    let mut f = File::open(format!("{}{}", filename, extension))
        .expect(format!("Failed to open shader file: {}{}", filename, extension).as_str());
    let mut file_bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut file_bytes)?;
    Ok(CString::new(file_bytes).expect(
        format!("Failed to read shader to CString: {}{}", filename, extension).as_str()))
}