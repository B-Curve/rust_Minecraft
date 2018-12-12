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

pub fn start(window: &mut Window, gl: &Gl) {
    let mut camera = Camera::new(vec3(0.0, 8.0, -2.0), vec3(0.0, 0.0, 1.0));
    let mut player = Player::new();
    let mut timer = Timer::new();
    let block_db = block_database::get();
    let mut world = World::new(gl, camera.position());
    player.set_position(camera.position());

    let mut shader = shader::Shader::new(
        gl, shader::Type::Block, false).unwrap();
    let mut torch_shader = shader::Shader::new(
        gl, shader::Type::Torch, false).unwrap();
    let mut lighting_shader = shader::Shader::new(
        gl, shader::Type::Deferred, false).unwrap();

    let lighting_buffer = DeferredBuffer::new(gl);

    shader.bind();

    lighting_shader.bind();
    lighting_shader.int("NR_LIGHTS", 1);
    lighting_shader.int("gPosition", 0);
    lighting_shader.int("gNormal", 1);
    lighting_shader.int("gAlbedoSpec", 2);

    let mut torches: Vec<(&Block, BlockBuffer, BlockLight, Vec3)> = Vec::new();

    let t = block_db.get_block(BlockType::JackOLantern);
    let tb = BlockBuffer::new(gl, t);
    let pos = vec3(8.0, 6.0, 12.0);
    torches.push((t, tb, t.get_light(&pos), pos));

    let torch_lights = torches.iter()
        .map(|(_, _, light, _)| light)
        .collect::<Vec<&BlockLight>>();
    let lights = torches.iter()
        .map(|(_, buffer, light, _)| (light, buffer))
        .collect::<Vec<(&BlockLight, &BlockBuffer)>>();

    while window.is_open() {
        timer.tick();
        window.process_events(gl);
        player.set_frame_leap(timer.frame_leap());
        player.update(&mut camera, window.get_window());

        lighting_buffer.bind_framebuffer();

        world.render(&shader, &camera);
        world.build_chunk_from_queue();

        lighting_buffer.unbind_framebuffer();
        lighting_buffer.apply_lighting(&player.position(), &torch_lights);
        lighting_buffer.copy_depth_buffer();
        world.bind_block_texture(0);
        lighting_buffer.render_lighting(&camera, &torch_shader, &lights);

        window.swap_buffers();
    }

}