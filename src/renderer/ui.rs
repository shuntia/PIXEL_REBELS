use crate::{
    assets::crosshair_tex,
    model::{GameMode, SAVE_PATHBUF_CACHE, Status, TitlePhase},
};
use macroquad::prelude::*;

pub async fn render_ui(status: &Status) {
    match &status.mode {
        GameMode::Title { phase } => render_title(phase),
        GameMode::Pause => render_pause_menu(),
        GameMode::Play => render_play(),
    }
}

fn render_play() {
    draw_text(
        &format!("time since: {}", get_frame_time()),
        0.,
        0.,
        40.,
        WHITE,
    );
    render_crosshair();
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
fn render_crosshair() {
    let mouse_pos = mouse_position();
    draw_texture_ex(
        &crosshair_tex,
        mouse_pos.0 - 25.,
        mouse_pos.1 - 25.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 50., y: 50. }),
            ..Default::default()
        },
    );
}
