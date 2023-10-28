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

const GRAVITY: f32 = 1.0;
const JUMP_HEIGHT: f32 = 3.0;

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
    };

    let mut player_animation_index: usize = 0;
    let mut frame_count = 0.0;
    let multiplier = 8.0;

    loop 
    {
        clear_background(WHITE);

        if frame_count % multiplier == 0.0
        {
            player_animation_index += 1;
        }

        player.update();
        player.draw(&mut player_animation_index);

        frame_count += 1.0;
        next_frame().await;
    }
}

/*****************************************************************************************/

// TRAITS
pub trait DrawableSingular
{
    fn draw(&self);
}

pub trait Animated {
    fn draw(&mut self, animation_index: &mut usize);
}

/*****************************************************************************************/

enum Direction {
    Left,
    Right,
}

enum PlayerState {
    Idle,
    Walk(Direction),
}

/// Stores player's location data, health, alive status
pub struct Player
{
    position: Vec2,
    health: u32,
    hitbox: Rect,
    texture: Texture2D,
    ghost_texture: Texture2D,
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

    
    // actions
    pub fn attack(&self) {
        // make hurt box appear in front of player
    }  
    pub fn handle_input(&mut self)
    {
        if is_key_down(KeyCode::Left)
        {
            self.position -= Vec2::X;
        }
        if is_key_down(KeyCode::Right)
        {
            self.position += Vec2::X;
        }
        if is_key_down(KeyCode::Z)
        {
            self.position.y -= JUMP_HEIGHT;
        }
    } 
    pub fn apply_gravity(&mut self)
    {
        if !(self.position.y > screen_height() + self.hitbox.y + self.hitbox.h)
        {
            self.position.y += GRAVITY;
        }
    }
    pub fn update(&mut self)
    {
        self.handle_input();
        self.apply_gravity();
    }
}
impl Animated for Player
{
    fn draw(&mut self, animation_index: &mut usize)
    {
        if *animation_index >= 9 
        {
            *animation_index = 0;
        }
        if self.health > 0
        {
            draw_texture_ex(
                &self.texture,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(170.0, 290.0)),
                    source: Some(PLAYER_IDLE_FRAMES[*animation_index]),
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
                    source: Some(PLAYER_IDLE_FRAMES[*animation_index]),
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
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
impl Animated for Enemy {
    fn draw(&mut self, animation_index: &mut usize) {
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

impl Animated for Platforms
{
    fn draw(&mut self, animation_index: &mut usize)
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

impl DrawableSingular for HealthHUD
{
    fn draw(& self)
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