use std::sync::LazyLock;

use async_std::{path::PathBuf, sync::RwLock};

use crate::errors::{GameError, Result};

static MAPID: LazyLock<RwLock<u32>> = LazyLock::new(|| RwLock::new(0));

pub async fn get_mapid() -> u32 {
    *MAPID.read().await
}

const MAPS: [&'static str; 1] = ["assets/map0"];

pub fn get_map(id: u32) -> Result<PathBuf> {
    MAPS.get(id as usize - 1)
        .ok_or(GameError::IllegalArgument(
            "Illegal index access on MAP.".into(),
        ))
        .map(|o| PathBuf::from(o))
}
