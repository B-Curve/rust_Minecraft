use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use freetype::{Library, Face};

pub fn load_font(filename: &str) -> Result<Face, String> {
    let lib = match Library::init() {
        Ok(l) => l,
        Err(e) => return Err(format!("Failed to initialize freetype: {}", e))
    };
    match lib.new_face(("./assets/fonts/".to_owned() + filename).as_str(), 0) {
        Ok(f) => Ok(f),
        Err(e) => Err(format!("Failed to load font: {}", e))
    }
}