use math;
use math::{One, vec2, vec3, Vec2, Vec3, Mat4, Geom::{normalize, cross, radians}, Ext as ext};

lazy_static! {
    static ref WORLD_UP: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
}

impl Camera {
    pub fn new(position: Vec3, front: Vec3, screen_size: (i32, i32)) -> Camera {
        let right = normalize(cross(front, *WORLD_UP));
        let up = normalize(cross(right, front));
        Camera {
            position,
            front,
            right,
            up,
            rotation: vec2(0.0, 0.0),
            mouse_pos: vec2(0.0, 0.0),
            mouse_speed: 0.05,
            model: Mat4::one(),
            view: ext::look_at(position, position + front, *WORLD_UP),
            projection: ext::perspective(radians(80.0), screen_size.0 as f32/screen_size.1 as f32, 0.1, 1000.0)
        }
    }

    pub fn update(&mut self, velocity: &Vec3, height_vector: &Vec3, mouse_x: f64, mouse_y: f64) {
        self.update_mouse(mouse_x, mouse_y);

        self.position = self.position + *velocity;

        self.view = ext::look_at(self.position + *height_vector, self.position + self.front + *height_vector, self.up);
    }

    pub fn update_mouse(&mut self, mouse_x: f64, mouse_y: f64) {
        let x = mouse_x as f32;
        let y = mouse_y as f32;

        if self.mouse_pos.x == x && self.mouse_pos.y == y { return; }

        let new_x = (x - self.mouse_pos.x) * self.mouse_speed;
        let new_y = (self.mouse_pos.y - y) * self.mouse_speed;

        self.rotation.x += new_x;
        self.rotation.y += new_y;

        if self.rotation.y > 89.9 { self.rotation.y = 89.9; }
        else if self.rotation.y < -89.9 { self.rotation.y = -89.9; }

        if self.rotation.x > 360.0 { self.rotation.x = 0.0; }
        else if self.rotation.x < 0.0 { self.rotation.x = 360.0; }

        self.front = normalize(vec3(
            self.rotation.x.to_radians().sin() * self.rotation.y.to_radians().cos(),
            self.rotation.y.to_radians().sin(),
            -self.rotation.x.to_radians().cos() * self.rotation.y.to_radians().cos()
        ));
        self.right = normalize(cross(self.front, *WORLD_UP));
        self.up = normalize(cross(self.right, self.front));

        self.mouse_pos.x = x;
        self.mouse_pos.y = y;
    }

    pub fn set_position(&mut self, pos: Vec3) { self.position = pos; }
    pub fn position(&self) -> &Vec3 { &self.position }
    pub fn forward(&self) -> &Vec3 { &self.front }
    pub fn right(&self) -> &Vec3 { &self.right }
    pub fn model(&self) -> &Mat4 { &self.model }
    pub fn view(&self) -> &Mat4 { &self.view }
    pub fn projection(&self) -> &Mat4 { &self.projection }
}

pub struct Camera {
    position: Vec3,
    front: Vec3,
    right: Vec3,
    up: Vec3,

    rotation: Vec2,
    mouse_pos: Vec2,
    mouse_speed: f32,

    model: Mat4,
    view: Mat4,
    projection: Mat4
}