use macroquad::prelude::*;

fn conf() -> Conf {
    Conf {
        window_title: String::from("CubicGame"),
        platform: miniquad::conf::Platform {
            // swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    cubic_game::run_client().await;
}
