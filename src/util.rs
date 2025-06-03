use std::f32::consts::PI;
use std::sync::Arc;
use std::{process::exit, sync::atomic::AtomicBool};

use async_std::sync::RwLock;
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use once_cell::sync::OnceCell;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::errors::{GameError, Nresult, Result};
use crate::model::World;
use crate::model::enemies::Enemy;
use crate::renderer::world::player_on_scr;

pub static INTERRUPT: AtomicBool = AtomicBool::new(false);

pub static DELTA_TIME: RwLock<f32> = RwLock::new(0.0);

pub async fn update_delta_time() -> Nresult {
    let mut lock=DELTA_TIME.try_write().ok_or(GameError::Unexpected("Should have been able to acquire RwLock! Are you sure you called it at the END of the frame?".into()))?;
    *lock = get_frame_time() as f32;
    Ok(())
}

pub static DEBUG_TX: OnceCell<Arc<UnboundedSender<String>>> = OnceCell::new();

pub async fn create_mpsc() -> crate::errors::Result<UnboundedReceiver<String>> {
    let (tx, rx) = mpsc::unbounded();
    DEBUG_TX
        .set(Arc::new(tx))
        .map_err(|_| GameError::Misc("Tried to create debug mpsc twice.".into()))?;
    Ok(rx)
}

pub fn set_hooks() {
    std::panic::set_hook(Box::new(|data| {
        INTERRUPT.store(true, std::sync::atomic::Ordering::Release);
        let payload = data.payload();
        let msg = if let Some(s) = payload.downcast_ref::<GameError>() {
            format!("{}", s)
        } else if let Some(s) = payload.downcast_ref::<String>() {
            format!("{}", s)
        } else if let Some(s) = payload.downcast_ref::<&'static str>() {
            format!("{}", s)
        } else {
            "Unknown Error.".to_owned()
        };
        let loc = data.location();
        native_dialog::DialogBuilder::message()
            .set_text(
            format!("FATAL ERROR:\n{msg}\nLocation:\n{loc:#?}\n\nPlease report this at https://github.com/shuntia/pixel_rebels"))
            .set_level(native_dialog::MessageLevel::Error)
            .set_title(":(")
            .alert()
            .show()
            .unwrap_or_else(|_|panic!("The dialog failed. I don't know why.\n\nMessage:\n{msg}\n\nLocation:\n{loc:#?}"));
        eprintln!("panicked:\n{:#?}\nbacktrace:\n{:#?}", msg, data.location());
    }));
    ctrlc::set_handler(|| {
        INTERRUPT.store(true, std::sync::atomic::Ordering::Release);
        native_dialog::DialogBuilder::message()
            .set_text("FATAL: control-c\n\nProcess killed by user")
            .set_title("X(")
            .set_level(native_dialog::MessageLevel::Error)
            .alert()
            .show()
            .unwrap_or_else(|_| {
                panic!("The dialog failed.\nFATAL: control-c\n\nProcess killed by user")
            });
        exit(130);
    })
    .unwrap_or_else(|_| error!("Failed to set ctrl-c hook."));
    unsafe {
        if signal_hook::low_level::register(15, || {
            INTERRUPT.store(true, std::sync::atomic::Ordering::Release);
            native_dialog::DialogBuilder::message()
                .set_text("FATAL: SIGTERM\n\nProcess killed by user")
                .set_title("X(")
                .set_level(native_dialog::MessageLevel::Error)
                .alert()
                .show()
                .unwrap_or_else(|_| {
                    panic!("The dialog failed.\nFATAL: SIGTERM\n\nProcess killed by user")
                });
            exit(133);
        })
        .is_err()
        {
            error!("Failed to set SIGTERM hook.");
        };
        if signal_hook::low_level::register(6, || {
            INTERRUPT.store(true, std::sync::atomic::Ordering::Release);
            native_dialog::DialogBuilder::message()
                .set_text("FATAL: SIGABRT\n\nProcess aborted itself.")
                .set_title(":O")
                .set_level(native_dialog::MessageLevel::Error)
                .alert()
                .show()
                .unwrap_or_else(|_| {
                    panic!("The dialog failed.\nFATAL: SIGABRT\n\nProcess aborted itself.")
                });
            exit(134);
        })
        .is_err()
        {
            error!("Failed to set SIGABRT hook.");
        };
    };
}

pub fn get_mouse_angle() -> f32 {
    ((Vec2::from(screen_size()) / 2. - Vec2::from(mouse_position())).to_angle() - PI).abs()
}
pub fn get_mouse_angle_centered(world: &World) -> f32 {
    ((crate::renderer::world::player_on_scr(world) - Vec2::from(mouse_position())).to_angle() - PI)
        .abs()
}

pub fn log_frame(s: String) {
    let _ = DEBUG_TX.get().unwrap().unbounded_send(s);
}

pub fn find_in_distance<'a>(
    enemies: &'a mut Vec<Enemy>,
    center: Vec2,
    dist: f32,
) -> Result<Vec<&'a mut Enemy>> {
    let find_y_center = center.y;
    let bottom = enemies
        .par_iter()
        .enumerate()
        .find_first(|(_, el)| el.loc.y >= find_y_center - dist);
    let top_idx;
    if let Some((found, _)) = bottom {
        if let Some((idx, _)) = enemies[found..]
            .par_iter()
            .enumerate()
            .find_first(|(_, el)| el.loc.y > find_y_center + dist)
        {
            top_idx = found + idx;
        } else {
            top_idx = enemies.len();
        }
        Ok(enemies[found..top_idx]
            .par_iter_mut()
            .filter(|el| el.loc.distance(center) <= dist)
            .collect())
    } else {
        Err(GameError::Unexpected("Failed to find any entities!".into()))
    }
}

#[derive(Debug, Clone, Copy, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PlayerAnimation {
    Idle = 0,
}
