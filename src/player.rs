use math::{Vec3, vec3};
use util::controls::{Controls, KeyState, KeyAction};
use std::ops::{Sub, Add};
use camera::Camera;

pub const CROUCH_HEIGHT_DECREASE: f32 = 0.44;
pub const FRAME_CONSTANT: f32 = 200.0;

impl Player {
    pub fn new() -> Player { Default::default() }

    pub fn position(&self) -> Vec3 { self.position }
    pub fn set_position(&mut self, p: &Vec3) { self.position = p.clone(); }

    pub fn height_vector(&self) -> Vec3 { vec3(0.0, self.height, 0.0) }

    pub fn set_frame_leap(&mut self, leap: u32) { self.frame_leap = leap; }

    pub fn is_sprinting(&self) -> bool { self.is_sprinting }
    pub fn set_sprinting(&mut self, is: bool) { self.is_sprinting = is; }

    pub fn is_crouched(&self) -> bool { self.is_crouched }
    pub fn set_crouched(&mut self, is: bool) { self.is_crouched = is; }

    pub fn move_speed(&self) -> f32 {
        (self.move_speed * match self.is_sprinting {
            true => self.sprint_scale,
            false => 1.0
        }) * self.frame_leap as f32 / FRAME_CONSTANT
    }

    pub fn back_speed(&self) -> f32 {
        self.move_speed * self.frame_leap as f32 / FRAME_CONSTANT
    }

    pub fn strafe_speed(&self) -> f32 {
        self.strafe_speed * self.frame_leap as f32 / FRAME_CONSTANT
    }

    pub fn update(&mut self, camera: &mut Camera, window: &::glfw::Window) {
        self.controls.update(window);
        let true_forward = self.true_froward();
        let true_back = self.true_back();
        let true_strafe = self.true_strafe();
        let mut height_vec = self.height_vector().clone();

        if self.controls.key_state(KeyAction::Forward) == &KeyState::Active {
            self.velocity = self.velocity.add(*camera.forward() * true_forward);
        }
        if self.controls.key_state(KeyAction::Back) == &KeyState::Active {
            self.velocity = self.velocity.sub(*camera.forward() * true_back);
        }
        if self.controls.key_state(KeyAction::Left) == &KeyState::Active {
            self.velocity = self.velocity.sub(*camera.right() * true_strafe);
        }
        if self.controls.key_state(KeyAction::Right) == &KeyState::Active {
            self.velocity = self.velocity.add(*camera.right() * true_strafe);
        }

        if self.controls.key_state(KeyAction::Crouch) == &KeyState::Active {
            self.set_crouched(true);
            height_vec.y -= CROUCH_HEIGHT_DECREASE;
        } else { self.set_crouched(false); }

        self.is_sprinting = self.controls.key_state(KeyAction::Sprint) == &KeyState::Active;

        let (x, y) = window.get_cursor_pos();
        camera.update(&self.velocity, &height_vec, x, y);
        self.velocity = ::math::Zero::zero();
        self.position = camera.position().clone();
    }

    fn true_froward(&self) -> f32 {
        (self.frame_leap as f32 / FRAME_CONSTANT) * (
            if self.is_sprinting { self.sprint_scale } else { 1.0 }
                * self.move_speed
                - (if self.is_crouched { self.crouch_slow } else { 0.0 })
        )
    }

    fn true_back(&self) -> f32 {
        (self.frame_leap as f32 / FRAME_CONSTANT) * (
            self.move_speed - (if self.is_crouched { self.crouch_slow } else { 0.0 })
        )
    }

    fn true_strafe(&self) -> f32 {
        (self.frame_leap as f32 / FRAME_CONSTANT) * (
            self.strafe_speed - (if self.is_crouched { self.crouch_slow } else { 0.0 })
        )
    }
}

pub struct Player {
    height: f32,
    width: f32,
    move_speed: f32,
    strafe_speed: f32,
    sprint_scale: f32,
    crouch_slow: f32,
    is_sprinting: bool,
    is_crouched: bool,
    jump_height: f32,
    frame_leap: u32,
    position: Vec3,
    velocity: Vec3,
    controls: Controls,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            height: 1.33,
            width: 0.5,
            move_speed: 1.13,
            strafe_speed: 0.93,
            sprint_scale: 1.44,
            crouch_slow: 0.77,
            is_sprinting: false,
            is_crouched: false,
            jump_height: 1.88,
            frame_leap: 0,
            position: ::math::Zero::zero(),
            velocity: ::math::Zero::zero(),
            controls: Controls::default(),
        }
    }
}