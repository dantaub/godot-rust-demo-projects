use crate::player;
use godot::classes::{Area2D, CollisionShape2D, IArea2D, Node2D, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Health {
    heal_amount: i64,
    base: Base<Area2D>,
}

#[godot_api]
impl Health {
    #[signal]
    pub fn collected(amount: i64);

    pub fn set_heal_amount(&mut self, amount: i64) {
        self.heal_amount = amount;
        let scale = if amount > 1 {
            Vector2::new(1.0, 1.0)
        } else {
            Vector2::new(0.5, 0.5)
        };
        let mut sprite = self.base().get_node_as::<Sprite2D>("Sprite2D");
        sprite.set_scale(scale);
        let mut shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");
        shape.set_scale(scale);
    }

    #[func]
    fn on_body_entered(&mut self, body: Gd<Node2D>) {
        if body.try_cast::<player::Player>().is_ok() {
            let amount = self.heal_amount;
            self.signals().collected().emit(amount);
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl IArea2D for Health {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            heal_amount: 1,
            base,
        }
    }

    fn ready(&mut self) {
        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);
    }
}
