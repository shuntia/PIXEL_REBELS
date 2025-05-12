use crate::model::{GameMode, GameModel};
use macroquad::prelude::*;

pub async fn render_ui(game: &GameModel) {
    match game.mode {
        GameMode::Menu => render_menu().await,
        GameMode::Pause => render_pause_menu().await,
        _ => {}
    }
}

async fn render_menu() {
    clear_background(BLACK);
    draw_text(
        "Horde Game",
        screen_width() / 2.0 - 100.0,
        screen_height() / 2.0 - 20.0,
        40.0,
        WHITE,
    );
    draw_text(
        "Press Enter to Start",
        screen_width() / 2.0 - 140.0,
        screen_height() / 2.0 + 20.0,
        30.0,
        GRAY,
    );
}

async fn render_pause_menu() {
    draw_rectangle(
        50.0,
        50.0,
        screen_width() - 100.0,
        screen_height() - 100.0,
        Color::new(0.0, 0.0, 0.0, 0.5),
    );
    draw_text(
        "Paused",
        screen_width() / 2.0 - 60.0,
        screen_height() / 2.0,
        50.0,
        YELLOW,
    );
}
