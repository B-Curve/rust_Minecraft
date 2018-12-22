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

use std::env;
use util::math;

struct VarArgs {
    width: u32,
    height: u32
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut var_args: VarArgs = VarArgs { width: 0, height: 0 };

    for arg in args.iter() {
        let key_val: Vec<&str> = arg.split("=").collect();
        let key = key_val[0];
        let val = key_val[1];

        if key.eq_ignore_ascii_case("width") {
            var_args.width = val.parse::<u32>()
                .unwrap_or_else(|_| {
                    println!("Invalid screen width set. Defaulting to 800");
                    800
                });
        }
        if key.eq_ignore_ascii_case("height") {
            var_args.height = val.parse::<u32>()
                .unwrap_or_else(|_| {
                    println!("Invalid screen height set. Defaulting to 600");
                    600
                });
        }
    }

    let mut window: window::Window = window::Window::new(var_args.width, var_args.height);
    let gl = window.gl_context();
    game::start(&mut window, &gl);
}
