use async_std::sync::RwLock;
use macroquad::prelude::*;

use crate::errors::{GameError, Nresult};

pub static DELTA_TIME: RwLock<f32> = RwLock::new(0.0);

pub async fn update_delta_time() -> Nresult {
    let mut lock=DELTA_TIME.try_write().ok_or(GameError::Unexpected("Should have been able to acquire RwLock! Are you sure you called it at the END of the frame?".into()))?;
    *lock = get_frame_time() as f32;
    Ok(())
}
