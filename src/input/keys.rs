use std::{collections::HashSet, time::Duration};

use async_std::task::sleep;
use macroquad::prelude::*;

pub struct KbdMan {
    keypress: HashSet<KeyCode>,
    keys: HashSet<KeyCode>,
    caught: HashSet<KeyCode>,
}

impl KbdMan {
    pub fn new() -> Self {
        KbdMan {
            keypress: HashSet::new(),
            keys: HashSet::new(),
            caught: HashSet::new(),
        }
    }
    pub fn update(&mut self) {
        self.keys = get_keys_down();
        self.keypress = get_keys_pressed();
        self.caught.clear();
    }
    pub fn keypress(&mut self, keycode: KeyCode) -> bool {
        if self.keypress.contains(&keycode) & !self.caught.contains(&keycode) {
            self.caught.insert(keycode);
            true
        } else {
            false
        }
    }
    pub fn keypress_unchecked(&mut self, keycode: KeyCode) -> bool {
        if self.keypress.contains(&keycode) {
            self.caught.insert(keycode);
            true
        } else {
            false
        }
    }
    pub fn keypress_peek(&mut self, keycode: KeyCode) -> bool {
        self.keypress.contains(&keycode) & !self.caught.contains(&keycode)
    }
    pub fn keypress_peek_unchecked(&mut self, keycode: KeyCode) -> bool {
        self.keypress.contains(&keycode)
    }
    pub fn keydown(&mut self, keycode: KeyCode) -> bool {
        self.keys.contains(&keycode)
    }
}
