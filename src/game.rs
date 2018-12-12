use GL;
use GL::Gl;
use window::Window;
use camera::Camera;
use player::Player;
use shader;
use util::{timer::Timer, math::{vec3, vec2, Vec3, Vec3i, Zero, One, Mat4, Ext::{scale, translate}}, vertex::Vertex};
use std::mem::{size_of_val, size_of};
use world::block::block_database;
use world::block::block_texture::BlockTexture;
use world::block::block_type::BlockType;
use world::block::block_buffer::BlockBuffer;
use render::deferred_buffer::DeferredBuffer;
use world::block::block::Block;
use world::block::block_light::BlockLight;
use world::chunk::chunk_buffer::ChunkBuffer;
use world::chunk::chunk::Chunk;
use world::constants::CHUNK_SIZE;
use world::world::World;
use std::rc::Rc;
use world::scenery::lighting::Lighting;
use util::text::Text;

pub fn start(window: &mut Window, gl: &Gl) {
    let mut camera = Camera::new(vec3(0.0, 8.0, -2.0), vec3(0.0, 0.0, 1.0));
    let mut player = Player::new();
    let mut timer = Timer::new();
    let mut writer = Text::from_font(gl, "archivo.ttf");
    let block_db = block_database::get();
    let mut world = World::new(gl, camera.position());
    let mut lighting = Lighting::new(gl);
    player.set_position(camera.position());

    let mut shader = shader::Shader::new(
        gl, shader::Type::Block, false).unwrap();

    lighting.add_light(BlockType::JackOLantern, vec3(2.0, 6.0, 2.0));
    lighting.add_light(BlockType::Torch, vec3(20.0, 6.0, 2.0));
    lighting.add_light(BlockType::JackOLantern, vec3(20.0, 6.0, 20.0));

    shader.bind();

    while window.is_open() {
        timer.tick();
        window.process_events(gl);
        player.set_frame_leap(timer.frame_leap());
        player.update(&mut camera, window.get_window());

        lighting.bind_framebuffer();

        world.render(&shader, &camera);
        world.build_chunk_from_queue();

        lighting.unbind_framebuffer();
        lighting.apply_lighting(&player.position());
        lighting.copy_depth_buffer();
        world.bind_block_texture(0);
        lighting.render_lighting(&camera);

        timer.draw_frames(&mut writer, &window);

        window.swap_buffers();
    }

}