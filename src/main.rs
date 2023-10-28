use macroquad::prelude::*;

#[macroquad::main("Mt Fuji")]
async fn main() 
{
    let slime = load_texture("./assets/slime.png").await.unwrap();
    let mut player_position = Vec2::new(screen_width(), screen_height());
    loop 
    {
        clear_background(WHITE);
        draw_texture(
            &slime,
            (screen_width() - slime.width()) / 2.0,
            (screen_height() - slime.height()) / 2.0,
            WHITE
        );
        next_frame().await;
    }
}