use macroquad::prelude::*;

mod player;
use player::Player;

// Fujisan, Sakuya-Sama, and the Sennin
// 富士山と咲夜様と仙人
#[macroquad::main(settings)]
async fn main() {
    let mut player = Player::new().await;
    loop {
        clear_background(LIGHTGRAY);

        player.test_animations();
        player.draw();

        next_frame().await;
    }
}

fn settings() -> Conf {
    return Conf {
        window_title: String::from("富士山と咲夜様と仙人"),
        ..Default::default()
    }
}
