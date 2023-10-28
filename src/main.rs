use macroquad::{prelude::*};
use livesplit_core::{Run, Segment, Timer, TimerPhase};

/*****************************************************************************************/

fn settings() -> Conf
{
    return Conf
    {
        window_title: String::from("究極の切腹クエスト～天魔の地上九峰～富士山3"),
        // window_width: todo!(),
        // window_height: todo!(),
        high_dpi: true,
        // fullscreen: todo!(),
        sample_count: 1,
        // window_resizable: todo!(),
        // icon: todo!(),
        // platform: todo!(),
        ..Default::default()
    }
}

/*****************************************************************************************/

const PLAYER_IDLE_FRAMES: [Rect; 10] = [
    Rect { x: (0.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (1.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (2.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (3.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (4.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (5.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (6.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (7.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (8.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
    Rect { x: (9.0 * (31.0 + 17.0) + 14.0),  y: 10.0, w: 20.0, h: 32.0 },
];

/*****************************************************************************************/

#[macroquad::main(settings)]
async fn main() 
{
    let player_texture = load_texture("assets/Asset Packs 1-3 (final)/Asset Pack-V1/Sprite Sheets/Character Idle 48x48.png").await.unwrap();
    let player_ghost_texture = load_texture("assets/slime.png").await.unwrap();
    let mut player = Player {
        texture: player_texture.clone(),
        position: Vec2::new(screen_width()/2.0, screen_height()/2.0),
        health: 2,
        hitbox: Rect::new(
            screen_width()/2.0,
            screen_height()/2.0,
            17.0 / 2.0,
            29.0 / 2.0,
        ),
        ghost_texture: player_ghost_texture,
        animation_index: 0,
    };

    let mut i: usize = 0;

    loop 
    {
        clear_background(WHITE);
        if is_key_pressed(KeyCode::Key0) { i = 0; }
        if is_key_pressed(KeyCode::Key1) { i = 1; }
        if is_key_pressed(KeyCode::Key2) { i = 2; }
        if is_key_pressed(KeyCode::Key3) { i = 3; }
        if is_key_pressed(KeyCode::Key4) { i = 4; }
        if is_key_pressed(KeyCode::Key5) { i = 5; }
        if is_key_pressed(KeyCode::Key6) { i = 6; }
        if is_key_pressed(KeyCode::Key7) { i = 7; }
        if is_key_pressed(KeyCode::Key8) { i = 8; }
        if is_key_pressed(KeyCode::Key9) { i = 9; }
        draw_texture_ex(
            &player_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(170.0, 290.0)),
                source: Some(PLAYER_IDLE_FRAMES[i]),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            }
        );
        // player.draw();
        next_frame().await;
    }
}

/*****************************************************************************************/

// TRAITS
pub trait Drawable
{
    fn draw(&mut self);
}

/*****************************************************************************************/

/// Stores player's location data, health, alive status
pub struct Player
{
    position: Vec2,
    health: u32,
    hitbox: Rect,
    texture: Texture2D,
    ghost_texture: Texture2D,
    animation_index: usize,
}
impl Player
{
    // getters
    pub fn get_position(&self) -> &Vec2
    {
        return &self.position;
    }
    pub fn get_health(&self) -> &u32
    {
        return &self.health;
    }
    pub fn get_hitbox(&self) -> &Rect
    {
        return &self.hitbox;
    }
    pub fn get_texture(&self) -> &Texture2D
    {
        return &self.texture;
    }
    pub fn get_ghost_texture(&self) -> &Texture2D
    {
        return &self.ghost_texture;
    }

    // setters
    pub fn set_position(&mut self, position: Vec2)
    {
        self.position = position;
    }
    pub fn set_health(&mut self, health: u32)
    {
        self.health = health;
    }
    pub fn set_hitbox(&mut self, hitbox: Rect)
    {
        self.hitbox = hitbox;
    }
    pub fn set_texture(&mut self, texture: Texture2D)
    {
        self.texture = texture;
    }
    pub fn set_ghost_texture(&mut self, ghost_texture: Texture2D)
    {
        self.ghost_texture = ghost_texture;
    }  

    pub fn attack(&self) {
        // make hurt box appear in front of player
    }  
}
impl Drawable for Player
{
    fn draw(&mut self)
    {
        if self.health > 0
        {
            draw_texture_ex(
                &self.texture,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(170.0, 290.0)),
                    source: Some(PLAYER_IDLE_FRAMES[self.animation_index]),
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        } else
        {
            draw_texture_ex(
                &self.ghost_texture,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(170.0, 290.0)),
                    source: Some(PLAYER_IDLE_FRAMES[self.animation_index]),
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        }

        if self.animation_index >= 9
        {
            self.animation_index = 0;
        }
        else
        {
            self.animation_index += 1;
        }
    }
}

struct Enemy {
    position: Vec2,
    health: u32,
    hitbox: Rect,
    hurtbox: Rect,
    texture: Texture2D,
    draw_params: DrawTextureParams
}
impl Enemy {
    // getters
    pub fn get_position(&self) -> &Vec2 
    {
        return &self.position;   
    }
    pub fn get_health(&self) -> &u32 
    {
        return &self.health;   
    }
    pub fn get_hitbox(&self) -> &Rect {
        return &self.hitbox;
    }
    pub fn get__hurtbox(&self) -> &Rect
    {
        return &self.hurtbox;
    }
    pub fn get_texture(&self) -> &Texture2D 
    {
        return &self.texture;
    }

    // setters
    pub fn set_position(&mut self, position: Vec2)
    {
        self.position = position;
    }
    pub fn set_health(&mut self, health: u32)
    {
        self.health = health;
    }
    pub fn set_hitbox(&mut self, hitbox: Rect)
    {
        self.hitbox = hitbox;
    }
    pub fn set_hurtbox(&mut self, hurtbox: Rect)
    {
        self.hurtbox = hurtbox;
    }
    pub fn set_texture(&mut self, texture: Texture2D)
    {
        self.texture = texture;
    }
}
impl Drawable for Enemy {
    fn draw(&mut self) {
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            self.draw_params.clone(),
        );
    }
}

/*****************************************************************************************/

// Platform object.

struct Platforms
{
    position: Vec2,
    texture: Texture2D,
    hitbox: Rect
}

impl Drawable for Platforms
{
    fn draw(&mut self)
    {
        // Draw platform texture
    }
}

/*****************************************************************************************/

// HealthHUD object

pub struct HealthHUD
{
    value: i32,
}

impl HealthHUD
{
    pub fn set(&mut self, value: i32)
    {
        self.value = value;
    }
    pub fn get(&self) -> i32
    {
        return self.value;
    }
}

impl Drawable for HealthHUD
{
    fn draw(&mut self)
    {
        // Draw background texture
        // Draw text label
    }
}

/*****************************************************************************************/

// Timer object
// Use livesplit core
fn timer() {
    let mut run = Run::new();
    
    run.set_game_name("Mt. Fuji");
    run.set_category_name("Any%");
    run.push_segment(Segment::new("Level 1"));
    
    let mut timer = Timer::new(run).expect("Run with at least one segment provided.");
    timer.start();
    timer.split();
    timer.reset(true);
}

/*****************************************************************************************/

// Menus
// Main
// Pause
// End Level
// End Game

/*****************************************************************************************/