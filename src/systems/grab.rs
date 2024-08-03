use macroquad::prelude::*;
use super::*;

use derive_more::{Deref, DerefMut};

pub struct TabPressSystem(());

#[derive(Deref, DerefMut)]
pub struct GrabbedState(pub bool);

impl System<&mut GrabbedState> for TabPressSystem {
    fn update(&self, grabbed: &mut GrabbedState) {
        if is_key_pressed(KeyCode::Tab) {
            grabbed.0 = !grabbed.0;
            set_cursor_grab(grabbed.0);
            show_mouse(!grabbed.0);
        }
    }
}

impl TabPressSystem {
    pub fn new() -> Self {
        set_cursor_grab(true);
        show_mouse(false);
        TabPressSystem(())
    }
}

impl GrabbedState {
    pub const fn new(state: bool) -> Self {
        Self(state)
    }
}

impl Default for GrabbedState {
    fn default() -> Self {
        Self(true)
    }
}






