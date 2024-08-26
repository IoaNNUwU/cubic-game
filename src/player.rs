use derive_more::{Deref, DerefMut};
use macroquad::prelude::*;

const LOOK_SPEED: f32 = 0.1;
const MOVE_SPEED: f32 = 0.1;

const UP: Vec3 = vec3(0.0, 1.0, 0.0);

/// ```
/// let mp: LastMousePos = mouse_position().into();
/// ```
#[derive(Deref, DerefMut, Clone, Copy)]
pub struct LastMousePos(pub Vec2);

impl From<(f32, f32)> for LastMousePos {
    fn from(value: (f32, f32)) -> Self {
        Self(value.into())
    }
}

impl From<Vec2> for LastMousePos {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct CurrentMousePos(pub Vec2);

impl From<(f32, f32)> for CurrentMousePos {
    fn from(value: (f32, f32)) -> Self {
        Self(value.into())
    }
}

impl From<Vec2> for CurrentMousePos {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

pub fn update_yaw_pitch_after_mouse_pos_changed(
    current_mouse_pos: &CurrentMousePos,
    last_mouse_pos: &LastMousePos,

    yaw: &mut Yaw,
    pitch: &mut Pitch,
) {
    let mouse_delta: Vec2 = current_mouse_pos.0 - last_mouse_pos.0;
    let frame_time = get_frame_time();

    yaw.0 += mouse_delta.x * frame_time * LOOK_SPEED;
    pitch.0 += mouse_delta.y * frame_time * -LOOK_SPEED;

    pitch.0 = if pitch.0 > 1.5 { 1.5 } else { pitch.0 };
    pitch.0 = if pitch.0 < -1.5 { -1.5 } else { pitch.0 };
}

#[derive(Deref, DerefMut, Clone, Copy, Default)]
pub struct PlayerPos(pub Vec3);

pub fn update_player_pos_after_front_right_up_changed(
    player_pos: &mut PlayerPos,

    front: Front,
    right: Right,
) {
    let mut velocity: Vec3 = Vec3::ZERO;

    if is_key_down(KeyCode::W) {
        velocity += zero_y_normalize(front.0);
    }
    if is_key_down(KeyCode::S) {
        velocity -= zero_y_normalize(front.0);
    }
    if is_key_down(KeyCode::A) {
        velocity -= zero_y_normalize(right.0);
    }
    if is_key_down(KeyCode::D) {
        velocity += zero_y_normalize(right.0);
    }
    if is_key_down(KeyCode::Space) {
        velocity += UP;
    }
    if is_key_down(KeyCode::LeftShift) {
        velocity -= UP;
    }

    if velocity != Vec3::ZERO {
        player_pos.0 += velocity.normalize() * MOVE_SPEED;
    }
}

fn zero_y_normalize(mut vec: Vec3) -> Vec3 {
    vec.y = 0.0;
    vec.normalize()
}

pub fn update_front_right_up_vecs_after_yaw_pitch_changed(
    yaw: &Yaw,
    pitch: &Pitch,

    front: &mut Front,
    right: &mut Right,
    up: &mut Up,
) {
    front.0 = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();

    right.0 = front.cross(UP).normalize();

    up.0 = right.cross(front.0).normalize();
}


#[derive(Debug, Deref, DerefMut, Clone, Copy)]
pub struct Front(pub Vec3);

impl Front {
    pub fn new(yaw: Yaw, pitch: Pitch) -> Self {
        Self(
            vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos(),
            )
            .normalize(),
        )
    }

    pub fn update(&mut self, yaw: Yaw, pitch: Pitch) {
        *self = Self::new(yaw, pitch);
    }
}

#[derive(Debug, Deref, DerefMut, Clone, Copy)]
pub struct Right(pub Vec3);

impl Right {
    pub fn new(front: Front) -> Self {
        Self(front.cross(UP).normalize())
    }

    pub fn update(&mut self, front: Front) {
        *self = Self::new(front);
    }
}

#[derive(Debug, Deref, DerefMut, Clone, Copy)]
pub struct Up(pub Vec3);

impl Up {
    pub fn new(right: Right, front: Front) -> Self {
        Self(right.cross(front.0).normalize())
    }

    pub fn update(&mut self, right: Right, front: Front) {
        *self = Self::new(right, front);
    }
}

/// ```
/// let yaw = Yaw::default();
/// ```
#[derive(Debug, Deref, DerefMut, Clone, Copy)]
pub struct Yaw(pub f32);

impl Default for Yaw {
    fn default() -> Self {
        Self(1.18)
    }
}

/// ```
/// let pitch = Pitch::default();
/// ```
#[derive(Debug, Deref, DerefMut, Clone, Copy)]
pub struct Pitch(pub f32);

impl Default for Pitch {
    fn default() -> Self {
        Self(0.0)
    }
}
