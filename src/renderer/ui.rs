use crate::model::{GameMode, SAVE_PATHBUF_CACHE, Status, TitlePhase};
use macroquad::prelude::*;

pub async fn render_ui(status: &Status) {
    match &status.mode {
        GameMode::Title { phase } => render_title(phase),
        GameMode::Pause => render_pause_menu(),
        GameMode::Play => (),
    }
}

fn render_title(phase: &TitlePhase) {
    match phase {
        TitlePhase::Start => {
            clear_background(BLACK);
            draw_text(
                "PIXEL_REBELS",
                screen_width() / 2.0 - 250.0,
                screen_height() / 2.0 - 20.0,
                100.0,
                WHITE,
            );
            draw_text(
                "Press Enter to Start",
                screen_width() / 2.0 - 140.0,
                screen_height() / 2.0 + 40.0,
                30.0,
                GRAY,
            );
        }
        TitlePhase::Menu(selection) => {
            render_menu(*selection);
        }
        _ => {
            clear_background(BLACK);
        }
    }
}

fn render_menu(selection: u32) {
    clear_background(BLACK);
    draw_text(
        "SAVE FILES",
        100.0,
        screen_height() / 2.0 - 100.0,
        100.0,
        WHITE,
    );
    draw_text(
        "NEW GAME",
        100.0,
        screen_height() / 2.0 + 20.0,
        30.0,
        if selection == 0 { YELLOW } else { WHITE },
    );
    let lock = SAVE_PATHBUF_CACHE.read().unwrap();
    for i in 0..lock.len() {
        draw_text(
            lock[i].file_name().unwrap().to_str().unwrap(),
            screen_width() / 2.0 - 100.0,
            screen_height() / 2.0 + i as f32 * 20.0,
            30.0,
            if selection == i as u32 + 1 {
                YELLOW
            } else {
                WHITE
            },
        );
    }
}

fn render_pause_menu() {
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
