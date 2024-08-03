
use macroquad::input::is_key_pressed;

use macroquad::prelude::*;
use super::*;

pub struct TabPressSystem(());

#[derive(derive_more::Deref, Default)]
pub struct GrabbedState(pub bool);

impl GrabbedState {
    pub fn new() -> Self {
        Self(true)
    }
}

impl System<&mut GrabbedState> for TabPressSystem {
    
    fn update(&self, grabbed: &mut GrabbedState) {
        if is_key_pressed(KeyCode::Tab) {
            grabbed.0 = !grabbed.0;
            set_cursor_grab(grabbed.0);
            show_mouse(!grabbed.0);
        }
    }
    
    fn init() -> Self {
        set_cursor_grab(true);
        show_mouse(false);
        TabPressSystem(())
    }
}








