use crate::math::Vector2;
use std::f64::consts::PI;

pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }

    pub fn update(&mut self) {
        for body in &mut self.bodies {
            body.update();
        }
    }

    pub fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for body in &self.bodies {
            body.draw(buffer, width, height);
        }
    }
}

impl RigidBody {
    pub fn new(position: Vector2, mass: f64) -> Self {
        Self {
            position,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            mass,
            inverse_mass: 1.0 / mass,
            inertia: 0.0,
            inverse_inertia: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.velocity = self.velocity.add(&self.acceleration);
        self.position = self.position.add(&self.velocity);
        self.acceleration = Vector2::new(0.0, 0.0);
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.acceleration = self.acceleration.add(&force.scale(self.inverse_mass));
    }

    pub fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        let x = self.position.x as usize;
        let y = self.position.y as usize;
        if x < width && y < height {
            buffer[y * width + x] = 0xFFFFFF; // White color
        }
    }
}