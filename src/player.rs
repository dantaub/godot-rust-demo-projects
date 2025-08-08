use crate::fireball;
use godot::classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Input, PackedScene};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    speed: real,
    screen_size: Vector2,
    fireball_scene: OnReady<Gd<PackedScene>>,
    #[export]
    invincibility_time: f64,
    direction: Vector2,
    last_horizontal: real,
    last_vertical: real,
    hit: bool,

    base: Base<Area2D>,
}

#[godot_api]
impl Player {
    // Public signal, since it's used by Main struct.
    #[signal]
    pub fn hit();

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<Node2D>) {
        if self.hit {
            return;
        }
        self.hit = true;
        self.base_mut().hide();
        self.signals().hit().emit();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_deferred("disabled", &true.to_variant());
        self.base_mut()
            .set_deferred("monitoring", &false.to_variant());
    }

    #[func]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_deferred("disabled", &false.to_variant());
        self.base_mut()
            .set_deferred("monitoring", &true.to_variant());
        self.hit = false;
    }

    pub fn respawn(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();

        let mut shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        // Defer disabling the collision shape to avoid "Can't change this state"
        // errors while the physics engine is flushing queries.
        shape.set_deferred("disabled", &true.to_variant());

        let mut timer = self
            .base()
            .get_tree()
            .unwrap()
            .create_timer(self.invincibility_time)
            .unwrap();
        timer.connect("timeout", &self.base().callable("enable_collision"));
    }

    #[func]
    fn enable_collision(&mut self) {
        let mut shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        // Re-enable the collision shape after the invincibility timer. Use
        // `set_deferred` to ensure it happens outside of the physics query flush.
        shape.set_deferred("disabled", &false.to_variant());
        self.base_mut()
            .set_deferred("monitoring", &true.to_variant());
        self.hit = false;
    }

    pub fn flash_red(&mut self) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.set_modulate(Color::from_rgba(1.0, 0.0, 0.0, 1.0));
        // Placeholder: play a hit animation when available.

        let mut timer = self.base().get_tree().unwrap().create_timer(0.2).unwrap();
        timer.connect("timeout", &self.base().callable("clear_flash"));
    }

    #[func]
    fn clear_flash(&mut self) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.set_modulate(Color::from_rgba(1.0, 1.0, 1.0, 1.0));
    }

    pub fn get_screen_size(&self) -> Vector2 {
        self.screen_size
    }
}

#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Player {
            speed: 400.0,
            screen_size: Vector2::new(0.0, 0.0),
            fireball_scene: OnReady::from_loaded("res://Fireball.tscn"),
            invincibility_time: 0.5,
            direction: Vector2::UP,
            last_horizontal: 1.0,
            last_vertical: -1.0,
            hit: false,
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        self.base_mut().hide();

        // Signal setup
        self.signals()
            .body_entered()
            .connect_self(Self::on_player_body_entered);
    }

    // `delta` can be f32 or f64; #[godot_api] macro converts transparently.
    fn process(&mut self, delta: f32) {
        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut velocity = Vector2::new(0.0, 0.0);

        // Note: exact=false by default, in Rust we have to provide it explicitly
        let input = Input::singleton();
        if input.is_action_pressed("move_right") {
            velocity += Vector2::RIGHT;
            self.last_horizontal = 1.0;
        }
        if input.is_action_pressed("move_left") {
            velocity += Vector2::LEFT;
            self.last_horizontal = -1.0;
        }
        if input.is_action_pressed("move_down") {
            velocity += Vector2::DOWN;
            self.last_vertical = 1.0;
        }
        if input.is_action_pressed("move_up") {
            velocity += Vector2::UP;
            self.last_vertical = -1.0;
        }

        if velocity.length() > 0.0 {
            self.direction = velocity.normalized();
            velocity = self.direction * self.speed;

            let animation;

            if velocity.x != 0.0 {
                animation = "right";

                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0)
            } else {
                animation = "up";

                animated_sprite.set_flip_v(velocity.y > 0.0)
            }

            animated_sprite.play_ex().name(animation).done();
        } else {
            animated_sprite.stop();
        }

        let change = velocity * delta;
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );
        self.base_mut().set_global_position(position);

        let input = Input::singleton();
        if input.is_action_just_pressed("shoot") {
            let mut parent_node = self.base().get_parent().unwrap();
            let mut fireball = self.fireball_scene.instantiate_as::<fireball::Fireball>();
            let direction = if self.last_vertical > 0.0 {
                Vector2::DOWN
            } else {
                Vector2::UP
            };
            let angular_speed = 10.0 * self.last_horizontal * -self.last_vertical;
            fireball.bind_mut().launch(direction, 600.0, angular_speed);
            fireball.set_global_position(self.base().get_global_position());
            fireball.connect("enemy_killed", &parent_node.callable("on_enemy_killed"));
            parent_node.add_child(&fireball);
        }
    }
}
