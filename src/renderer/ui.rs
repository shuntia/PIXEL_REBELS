use std::sync::atomic::AtomicBool;

use crate::{
    assets::CROSSHAIR_TEX,
    errors::Nresult,
    model::{GameMode, GameModel, SAVE_PATHBUF_CACHE, Status, TitlePhase, player::Player},
};
use futures::channel::mpsc::UnboundedReceiver;
use macroquad::prelude::*;

pub static LOG: AtomicBool = AtomicBool::new(false);

pub async fn render_ui(status: &Status, player: &Player) {
    match &status.mode {
        GameMode::Title { phase } => render_title(phase),
        GameMode::Pause => render_pause_menu(),
        GameMode::Play => render_play(player),
        GameMode::GameOver => render_gameover(),
    }
}

const DEBUG_FONT_SIZE: f32 = 50.;

pub fn render_dbg(rx: &mut UnboundedReceiver<String>) -> Nresult {
    if !LOG.load(std::sync::atomic::Ordering::Relaxed) {
        return Ok(());
    }
    let mut targets = Vec::new();
    while let Ok(Some(s)) = rx.try_next() {
        targets.push(s);
    }
    for i in 0..targets.len() {
        draw_text(
            &targets[i],
            0.,
            i as f32 * DEBUG_FONT_SIZE + 75. + DEBUG_FONT_SIZE * i as f32,
            DEBUG_FONT_SIZE,
            WHITE,
        );
    }
    Ok(())
}

static WAS_PAUSE: AtomicBool = AtomicBool::new(false);

fn render_play(model: &Player) {
    if WAS_PAUSE.load(std::sync::atomic::Ordering::Relaxed) {
        miniquad::window::show_mouse(false);
        WAS_PAUSE.store(false, std::sync::atomic::Ordering::Release);
    }
    render_health(model);
    render_crosshair();
}

fn render_title(phase: &TitlePhase) {
    match phase {
        TitlePhase::Start => {
            clear_background(BLACK);
            draw_text(
                "APCS_FINAL",
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

fn render_health(player: &Player) {
    draw_text("Health: ", 10., screen_height() - 50., 30., WHITE);
    let xoffset = 100.;
    draw_rectangle(
        xoffset,
        screen_height() - 50.,
        (xoffset + screen_width()) * (player.health / player.max_health),
        50.,
        GREEN,
    );
}

fn render_gameover() {
    clear_background(BLACK);
    draw_text("GAME OVER", 100., screen_height() / 2.0 - 100., 100., WHITE);
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
    if !WAS_PAUSE.load(std::sync::atomic::Ordering::Relaxed) {
        WAS_PAUSE.store(true, std::sync::atomic::Ordering::Release);
        miniquad::window::show_mouse(true);
    }
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
        &CROSSHAIR_TEX,
        mouse_pos.0 - 25.,
        mouse_pos.1 - 25.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 { x: 50., y: 50. }),
            ..Default::default()
        },
    );
}
