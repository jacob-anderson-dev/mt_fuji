use macroquad::prelude::*;

#[macroquad::main("Mt Fuji")]
async fn main() 
{
    let slime = load_texture("").await.unwrap();
    loop 
    {
        clear_background(BLACK);
        next_frame().await;
    }
}