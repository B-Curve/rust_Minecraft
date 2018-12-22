use camera::Camera;
use player::Player;
use shader;
use util::text::Text;
use util::{math::vec3, timer::Timer};
use window::Window;
use world::block::block_type::BlockType;
use world::scenery::lighting::Lighting;
use world::scenery::skybox::SkyBox;
use world::world::World;
use GL::Gl;

pub fn start(window: &mut Window, gl: &Gl) {
    let mut camera = Camera::new(vec3(0.0, 120.0, -2.0), vec3(0.0, 0.0, 1.0), window.framebuffer_size());
    let mut player = Player::new();
    let mut timer = Timer::new();
    let mut writer = Text::from_font(gl, "archivo.ttf", window.framebuffer_size());
    let mut world = World::new(gl);
    let mut lighting = Lighting::new(gl, window.framebuffer_size());
    let skybox = SkyBox::new(gl);
    player.set_position(camera.position());

    world.initialize_chunks();

    let shader = shader::Shader::new(gl, shader::Type::Block, false).unwrap();

    lighting.add_light(BlockType::Sun, vec3(0.0, 256.0, 0.0));

    shader.bind();

    'load_spawn: loop {
        if let Some(pos) = *world.get_player_spawn() {
            camera.set_position(pos);
            break 'load_spawn;
        } else {
            continue;
        }
    }

    while window.is_open() {
        timer.tick();
        window.process_events(gl);
        player.set_frame_leap(timer.frame_leap());
        player.update(&mut camera, window.get_window());
        
        world.render(&shader, &camera);

        lighting.bind_framebuffer();

        world.build_chunks(&player.position());
        world.render(&shader, &camera);
        world.take_chunk_from_queue();

        lighting.unbind_framebuffer();
        lighting.apply_lighting(&player.position());
        lighting.copy_depth_buffer();
        world.bind_block_texture(0);
        lighting.render_lighting(&camera);

        skybox.draw(&camera.view(), &camera.projection());
        timer.draw_frames(&mut writer, &window);
        player.draw_position(&mut writer);

        window.swap_buffers();
    }
}
