use macroquad::prelude::*;
use derive_more::{Deref, DerefMut};

const DEFAULT_GRABBED: bool = true;

#[derive(Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Grabbed(pub bool);

impl Grabbed {
    pub const fn new(grabbed: bool) -> Self {
        Self(grabbed)
    }

    pub fn switch(&mut self) {
        self.0 = !self.0
    }
}

impl Default for Grabbed {
    fn default() -> Self {
        Self(DEFAULT_GRABBED)
    }
}

impl PartialEq<bool> for Grabbed {
    fn eq(&self, other: &bool) -> bool {
        self.0 == *other
    }
}

pub fn update_grabbed_state_and_cursor_on_tab_press(grabbed: &mut Grabbed) {
    if is_key_pressed(KeyCode::Tab) {
        grabbed.switch();
        set_cursor_grab(grabbed.0);
        show_mouse(!grabbed.0);
    }
}

pub fn setup_mouse_cursor() {
    set_cursor_grab(DEFAULT_GRABBED);
    show_mouse(!DEFAULT_GRABBED);
}
