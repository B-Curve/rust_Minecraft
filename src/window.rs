use GL;
use GL::Gl;
use glfw;
use glfw::{Action, Context, Key};
use std::sync::mpsc::Receiver;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    is_open: bool
}

impl Window {
    pub fn new() -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        #[cfg(target_os = "macos")]
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(1920, 1080, "Game", glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        glfw.with_primary_monitor_mut(|_: &mut _, m: Option<&glfw::Monitor>| {
            let monitor = m.unwrap();
            let (x, y) = monitor.get_physical_size();
            let mode: glfw::VidMode = monitor.get_video_mode().unwrap();
            window.set_monitor(glfw::WindowMode::Windowed, x, y, WIDTH,
                               HEIGHT, Some(mode.refresh_rate));
        });

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_enter_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);
        window.set_sticky_keys(true);

        Window {
            glfw,
            window,
            events,
            is_open: true,
        }
    }

    pub fn gl_context(&mut self) -> Gl {
        unsafe {
            let gl = Gl::load_with(|s| self.window.get_proc_address(s) as *const _);
            &gl.BlendFunc(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
            &gl.FrontFace(GL::CCW);
            &gl.Enable(GL::CULL_FACE);
            &gl.CullFace(GL::BACK);
            &gl.Enable(GL::DEPTH_TEST);
            &gl.DepthFunc(GL::LESS);
            gl
        }
    }

    pub fn size(&self) -> (i32, i32) {
        self.window.get_size()
    }

    pub fn process_events(&mut self, gl: &Gl) {
        self.glfw.poll_events();

        unsafe {
            gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl.Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.is_open = false,
                _ => {}
            }
        }
        if self.window.should_close() {
            self.is_open = false;
        }
    }

    pub fn get_window(&self) -> &glfw::Window {
        &self.window
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn is_open(&self) -> bool { self.is_open }
}