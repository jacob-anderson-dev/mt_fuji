use macroquad::{experimental::animation::*, prelude::*};

const PLAYER_SPEED: f32 = 1.0;
const PLAYER_SIZE: f32 = 5.0;

#[derive(Default)]
enum PlayerState {
    #[default]
    Idle,
    HeavyAttack,
    LightAttack,
    Jump,
    Roll,
    Run,
    RunSword,
    TakeDamage,
    Death,
}

// TODO: make all animations on one large texture. This will eliminate the need for multiple textures and animated sprites
struct PlayerTextures {
    idle: Texture2D,
    heavy_attack: Texture2D,
    light_attack: Texture2D,
    jump: Texture2D,
    roll: Texture2D,
    run: Texture2D,
    run_sword: Texture2D,
    take_damage: Texture2D,
    death: Texture2D,
}
impl PlayerTextures {
    pub async fn new() -> Self {
        let player_textures = PlayerTextures {
            idle: load_texture("assets/samurai/idle.png").await.expect("FAILED TO LOAD: assets/samurai/idle.png"),
            heavy_attack: load_texture("assets/samurai/heavy-attack.png").await.expect("FAILED TO LOAD: assets/samurai/heavy-attack.png"),
            light_attack: load_texture("assets/samurai/light-attack.png").await.expect("FAILED TO LOAD: assets/samurai/light-attack.png"),
            jump: load_texture("assets/samurai/jump.png").await.expect("FAILED TO LOAD: assets/samurai/jump.png"),
            roll: load_texture("assets/samurai/roll.png").await.expect("FAILED TO LOAD: assets/samurai/roll.png"),
            run: load_texture("assets/samurai/run.png").await.expect("FAILED TO LOAD: assets/samurai/run.png"),
            run_sword: load_texture("assets/samurai/run-sword.png").await.expect("FAILED TO LOAD: assets/samurai/run-sword.png"),
            take_damage: load_texture("assets/samurai/take-dmg.png").await.expect("FAILED TO LOAD: assets/samurai/take-dmg.png"),
            death: load_texture("assets/samurai/death.png").await.expect("FAILED TO LOAD: assets/samurai/death.png"),
        };
        player_textures.filter(FilterMode::Nearest);
        return player_textures;
    }
    pub fn filter(&self, filter: FilterMode) {
        self.idle.set_filter(filter);
        self.heavy_attack.set_filter(filter);
        self.light_attack.set_filter(filter);
        self.jump.set_filter(filter);
        self.roll.set_filter(filter);
        self.run.set_filter(filter);
        self.run_sword.set_filter(filter);
        self.take_damage.set_filter(filter);
        self.death.set_filter(filter);
    }
}

struct PlayerSprites {
    idle: AnimatedSprite,
    heavy_attack: AnimatedSprite,
    light_attack: AnimatedSprite,
    jump: AnimatedSprite,
    roll: AnimatedSprite,
    run: AnimatedSprite,
    run_sword: AnimatedSprite,
    take_damage: AnimatedSprite,
    death: AnimatedSprite,    
}
impl PlayerSprites {
    pub fn new() -> Self {
        PlayerSprites {
            idle: AnimatedSprite::new(
                30,
                36,
                &[Animation { name: String::from("idle"), row: 0, frames: 3, fps: 4, }],
                true,
            ),
            heavy_attack: AnimatedSprite::new(
                56,
                36,
                &[Animation { name: String::from("heavy_attack"), row: 0, frames: 11, fps: 20 }],
                true,
            ),
            light_attack: AnimatedSprite::new(
                44,
                36,
                &[Animation { name: String::from("light_attack"), row: 0, frames: 7, fps: 20 }],
                true,
            ),
            jump: AnimatedSprite::new(
                22,
                36,
                &[Animation { name: String::from("jump"), row: 0, frames: 4, fps: 20 }],
                true,
            ),
            roll: AnimatedSprite::new(
                32,
                36,
                &[Animation { name: String::from("roll"), row: 0, frames: 6, fps: 20 }],
                true,
            ),
            run: AnimatedSprite::new(
                24,
                36,
                &[Animation { name: String::from("run"), row: 0, frames: 12, fps: 20, }],
                true,
            ),
            run_sword: AnimatedSprite::new(
                32,
                36,
                &[Animation { name: String::from("run_sword"), row: 0, frames: 10, fps: 20 }],
                true,
            ),
            take_damage: AnimatedSprite::new(
                32,
                36,
                &[Animation { name: String::from("take_damage"), row: 0, frames: 2, fps: 20 }],
                true,
            ),
            death: AnimatedSprite::new(
                32,
                36,
                &[Animation { name: String::from("death"), row: 0, frames: 11, fps: 20 }],
                true,
            ),
        }
    }
}
pub struct Player {
    textures: PlayerTextures,
    sprites: PlayerSprites,
    state: PlayerState,
    x: f32, // between his feet?
    y: f32, // THE TOP OF HIS HEAD?
    facing_left: bool,
}
impl Player {
    pub async fn new() -> Player {
        return Player {
            textures: PlayerTextures::new().await,
            sprites: PlayerSprites::new(),
            state: PlayerState::default(),
            x: 0.0,
            y: 0.0,
            facing_left: false,
        }
    }

    fn update_active_sprite(&mut self) {
        match self.state {
            PlayerState::Idle => self.sprites.idle.update(),
            PlayerState::HeavyAttack => self.sprites.heavy_attack.update(),
            PlayerState::LightAttack => self.sprites.light_attack.update(),
            PlayerState::Jump => self.sprites.jump.update(),
            PlayerState::Roll => self.sprites.roll.update(),
            PlayerState::Run => self.sprites.run.update(),
            PlayerState::RunSword => self.sprites.run_sword.update(),
            PlayerState::TakeDamage => self.sprites.take_damage.update(),
            PlayerState::Death => self.sprites.death.update(),
        };
    }

    fn get_animation_data(&self) -> (&Texture2D, &AnimatedSprite) {
        return match self.state {
            PlayerState::Idle => (&self.textures.idle, &self.sprites.idle),
            PlayerState::HeavyAttack => (&self.textures.heavy_attack, &self.sprites.heavy_attack),
            PlayerState::LightAttack => (&self.textures.light_attack, &self.sprites.light_attack),
            PlayerState::Jump => (&self.textures.jump, &self.sprites.jump),
            PlayerState::Roll => (&self.textures.roll, &self.sprites.roll),
            PlayerState::Run => (&self.textures.run, &self.sprites.run),
            PlayerState::RunSword => (&self.textures.run_sword, &self.sprites.run_sword),
            PlayerState::TakeDamage => (&self.textures.take_damage, &self.sprites.take_damage),
            PlayerState::Death => (&self.textures.death, &self.sprites.death),
        };
    }
    
    pub fn draw(&mut self) {
        let (texture, sprite) = self.get_animation_data();
        
        draw_texture_ex(
            texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                source: Some(sprite.frame().source_rect),
                dest_size: Some(sprite.frame().dest_size * PLAYER_SIZE),
                flip_x: self.facing_left,
                ..Default::default()
            }
        );

        self.update_active_sprite();
    }

    pub fn test_animations(&mut self) {
        if is_key_down(KeyCode::Key1) {
            self.state = PlayerState::Idle;
        }
        if is_key_down(KeyCode::Key2) {
            self.state = PlayerState::HeavyAttack;
        }
        if is_key_down(KeyCode::Key3) {
            self.state = PlayerState::LightAttack;
        }
        if is_key_down(KeyCode::Key4) {
            self.state = PlayerState::Jump;
        }
        if is_key_down(KeyCode::Key5) {
            self.state = PlayerState::Roll;
        }
        if is_key_down(KeyCode::Key6) {
            self.state = PlayerState::Run;
        }
        if is_key_down(KeyCode::Key7) {
            self.state = PlayerState::RunSword;
        }
        if is_key_down(KeyCode::Key8) {
            self.state = PlayerState::TakeDamage;
        }
        if is_key_down(KeyCode::Key9) {
            self.state = PlayerState::Death;
        }
    }
    
    pub fn handle_input(&mut self) {
        if is_key_down(KeyCode::Right) {
            self.facing_left = false;
            self.state = PlayerState::RunSword;
        }
        if is_key_down(KeyCode::Left) {
            self.facing_left = true;
            self.state = PlayerState::RunSword;
        }
        if is_key_down(KeyCode::Z) {
            self.state = PlayerState::HeavyAttack;
        }
        if is_key_down(KeyCode::X) {
            self.state = PlayerState::Jump;
        }
        if is_key_down(KeyCode::C) {
            self.state = PlayerState::LightAttack;
        }
    }
}