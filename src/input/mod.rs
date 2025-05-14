pub mod keys;
pub mod mouse;

pub struct InputMan {
    pub kbd: keys::KbdMan,
}

impl InputMan {
    pub fn new() -> Self {
        InputMan {
            kbd: keys::KbdMan::new(),
        }
    }
}
