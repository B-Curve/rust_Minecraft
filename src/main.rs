extern crate gl as GL;
extern crate freetype;
extern crate glfw;
extern crate glm;
#[macro_use] extern crate lazy_static;
extern crate stb_image;
extern crate libc;
extern crate rand;
extern crate noise;
extern crate num_traits;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate threadpool;

mod game;
mod window;
mod shader;
#[macro_use] mod util;
mod camera;
mod world;
mod player;
mod render;

use util::math;

fn main() {
    let mut window: window::Window = window::Window::new();
    let gl = window.gl_context();
    game::start(&mut window, &gl);
}
