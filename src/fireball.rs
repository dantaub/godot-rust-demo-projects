use godot::classes::{Area2D, IArea2D, Node2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Fireball {
    velocity: Vector2,
    angular_speed: real,
    hit: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl Fireball {
    #[signal]
    pub fn enemy_killed();

    pub fn launch(&mut self, direction: Vector2, speed: real, angular_speed: real) {
        let direction = direction.normalized();
        self.velocity = direction * speed;
        self.angular_speed = angular_speed;
        self.base_mut().set_rotation(direction.angle());
        self.hit = false;
    }

    #[func]
    fn on_body_entered(&mut self, mut body: Gd<Node2D>) {
        if self.hit {
            return;
        }
        self.hit = true;
        if body.is_in_group("mobs") {
            body.queue_free();
            self.signals().enemy_killed().emit();
        }
        self.base_mut()
            .set_deferred("monitoring", &false.to_variant());
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IArea2D for Fireball {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            velocity: Vector2::ZERO,
            angular_speed: 0.0,
            hit: false,
            base,
        }
    }

    fn ready(&mut self) {
        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);

        let mut timer = self.base().get_tree().unwrap().create_timer(3.0).unwrap();
        timer.connect("timeout", &self.base().callable("queue_free"));
    }

    fn physics_process(&mut self, delta: f64) {
        let delta = real::from_f64(delta);
        let rot = self.angular_speed * delta;
        self.velocity = self.velocity.rotated(rot);
        let change = self.velocity * delta;
        let position = self.base().get_global_position() + change;
        self.base_mut().set_global_position(position);
        let rotation = self.base().get_rotation() + rot;
        self.base_mut().set_rotation(rotation);
    }
}
