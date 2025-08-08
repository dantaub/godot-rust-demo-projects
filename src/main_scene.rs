use crate::{health, hud, mob, player};

use godot::classes::{AudioStreamPlayer, Marker2D, PathFollow2D, RigidBody2D, Timer};
use godot::prelude::*;

use rand::Rng as _;
use std::f32::consts::PI;

// Deriving GodotClass makes the class available to Godot.
#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    mob_scene: OnReady<Gd<PackedScene>>,
    health_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<player::Player>>,
    hud: OnReady<Gd<hud::Hud>>,
    music: OnReady<Gd<AudioStreamPlayer>>,
    death_sound: OnReady<Gd<AudioStreamPlayer>>,
    score: i64,
    health: i64,
    kill_count: i64,
    next_health_kills: i64,
    #[export]
    starting_health: i64,
    base: Base<Node>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        // We could also initialize those manually inside ready(), but OnReady automatically defers initialization.
        // Alternatively to init(), you can use #[init(...)] on the struct fields.
        Self {
            // OnReady::from_loaded(path) == OnReady::new(|| tools::load(path)).
            mob_scene: OnReady::from_loaded("res://Mob.tscn"),
            health_scene: OnReady::from_loaded("res://Health.tscn"),
            player: OnReady::from_node("Player"),
            hud: OnReady::from_node("Hud"),
            music: OnReady::from_node("Music"),
            death_sound: OnReady::from_node("DeathSound"),
            score: 0,
            health: 0,
            kill_count: 0,
            next_health_kills: 0,
            starting_health: 4,
            base,
        }
    }

    fn ready(&mut self) {
        // The OnReady instances are now initialized, we can access them like normal fields.

        // Get a Gd<Main> pointer to this instance.
        let main = self.to_gd();

        // Connect Player::hit -> Main::on_player_hit.
        self.player
            .signals()
            .hit()
            .connect_other(&main, Self::on_player_hit);

        // Connect Hud::start_game -> Main::new_game.
        self.hud
            .signals()
            .start_game()
            .connect_other(&main, Self::new_game);

        // Connect Main.MobTimer::timeout -> Main::on_mob_timer_timeout.
        self.mob_timer()
            .signals()
            .timeout()
            .connect_other(&main, Self::on_mob_timer_timeout);

        // Main.StartTimer::timeout -> Main::on_start_timer_timeout is set up in the Editor's Inspector UI, but could be done here as well,
        // as follows. Note that signal handlers connected via Rust do not need a #[func] annotation, they can remain entirely visible to Godot.
        //
        // self.start_timer()
        //     .signals()
        //     .timeout()
        //     .connect_other(&main, Self::on_start_timer_timeout);
    }
}

#[godot_api]
impl Main {
    // No #[func] here, this method is directly called from Rust (via type-safe signals).
    fn game_over(&mut self) {
        self.mob_timer().stop();

        self.hud.bind_mut().show_game_over();

        self.music.stop();
        self.death_sound.play();
    }

    // No #[func].
    pub fn new_game(&mut self) {
        let start_position = self.base().get_node_as::<Marker2D>("StartPosition");

        self.score = 0;
        self.health = self.starting_health;
        self.kill_count = 0;
        self.next_health_kills = rand::thread_rng().gen_range(6..=14);

        self.player.bind_mut().start(start_position.get_position());
        self.start_timer().start();

        let hud = self.hud.bind_mut();
        hud.update_score(self.score);
        hud.update_health(self.health);
        hud.show_message("Get Ready".into());

        self.music.play();
    }

    #[func] // needed because connected in Editor UI (see ready).
    fn on_start_timer_timeout(&mut self) {
        self.mob_timer().start();
    }

    pub fn on_player_hit(&mut self) {
        if self.health > 0 {
            self.health -= 1;
        }
        self.hud.bind_mut().update_health(self.health);

        if self.health <= 0 {
            self.game_over();
        } else {
            let mut player = self.player.bind_mut();
            let pos = player.base().get_global_position();
            player.respawn(pos);
            player.flash_red();
        }
    }

    #[func]
    fn on_enemy_killed(&mut self) {
        self.score += 1;
        self.hud.bind_mut().update_score(self.score);

        self.kill_count += 1;
        if self.kill_count >= self.next_health_kills {
            self.spawn_health();
            self.kill_count = 0;
            self.next_health_kills = rand::thread_rng().gen_range(6..=14);
        }
    }

    // No #[func], connected in pure Rust.
    fn on_mob_timer_timeout(&mut self) {
        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        // Instantiate the mob scene.
        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();

        let mut rng = rand::thread_rng();
        let progress = rng.gen_range(u32::MIN..u32::MAX);

        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + PI / 2.0;
        direction += rng.gen_range(-PI / 4.0..PI / 4.0);

        mob_scene.set_rotation(direction);

        self.base_mut().add_child(&mob_scene);

        let mut mob = mob_scene.cast::<mob::Mob>();
        let range = {
            // Local scope to bind `mob` user object
            let mob = mob.bind();
            rng.gen_range(mob.min_speed..mob.max_speed)
        };

        mob.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));
    }

    // These timers could also be stored as OnReady fields, but are now fetched via function for demonstration purposes.
    fn start_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("StartTimer")
    }

    fn mob_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("MobTimer")
    }

    fn spawn_health(&mut self) {
        let mut health_pickup = self.health_scene.instantiate_as::<health::Health>();
        let mut rng = rand::thread_rng();
        let screen_size = self.player.bind().get_screen_size();
        let x = rng.gen_range(0.0..screen_size.x);
        let y = rng.gen_range(0.0..screen_size.y);
        health_pickup.set_global_position(Vector2::new(x, y));

        let amount = if rng.gen_bool(0.1) { 3 } else { 1 };
        health_pickup.bind_mut().set_heal_amount(amount);

        let main = self.to_gd();
        health_pickup.connect("collected", &main.callable("on_health_collected"));
        self.base_mut().add_child(&health_pickup);
    }

    #[func]
    fn on_health_collected(&mut self, amount: i64) {
        self.health = (self.health + amount).min(self.starting_health);
        self.hud.bind_mut().update_health(self.health);
    }
}
