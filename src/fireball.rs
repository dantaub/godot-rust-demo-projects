use godot::classes::{Area2D, IArea2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Fireball {
    velocity: Vector2,
    angular_speed: real,
    base: Base<Area2D>,
}

#[godot_api]
impl Fireball {
    #[signal]
    pub fn enemy_killed();

    pub fn launch(&mut self, direction: Vector2, speed: real, angular_speed: real) {
        self.velocity = direction.normalized() * speed;
        self.angular_speed = angular_speed;
    }

    #[func]
    fn on_body_entered(&mut self, mut body: Gd<Node2D>) {
        if body.is_in_group("mobs") {
            body.queue_free();
            self.signals().enemy_killed().emit();
        }
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IArea2D for Fireball {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            velocity: Vector2::ZERO,
            angular_speed: 0.0,
            base,
        }
    }

    fn ready(&mut self) {
        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);
    }

    fn physics_process(&mut self, delta: f64) {
        let rot = self.angular_speed * real::from_f64(delta);
        self.velocity = self.velocity.rotated(rot);
        let change = self.velocity * real::from_f64(delta);
        self.base_mut().translate(change);
    }
}
