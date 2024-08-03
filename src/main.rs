mod physics;
mod math;

use minifb::{Key, Window, WindowOptions};
use physics::{PhysicsWorld, RigidBody};
use math::Vector2;


fn main() {
    let mut world = PhysicsWorld::new();
    let mut window = Window::new("Physics Simulator - Viktor Svensson", 800, 600, WindowOptions::default()).unwrap();

    let mut body_a = RigidBody::new(Vector2::new(100.0, 100.0), 1.0);
    let mut body_b = RigidBody::new(Vector2::new(500.0, 500.0), 1.0);

    world.add_body(body_a);
    world.add_body(body_b);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        world.update();
        window.update_with_buffer(&world.draw(), 800, 600).unwrap();
    }
}
