use super::*;
use macroquad::prelude::*;

use derive_more::{Deref, DerefMut};

const LOOK_SPEED: f32 = 0.1;
const MOVE_SPEED: f32 = 0.1;

const UP: Vec3 = vec3(0.0, 1.0, 0.0);

/// # System
pub struct UpdateYawPitch;

pub struct UpdateYawPitchArgs<'yp> {
    pub yaw_pitch: &'yp mut YawPitch,
    pub mouse_pos: MousePos,
    pub last_mouse_pos: LastMousePos,
}

impl<'yp> System<UpdateYawPitchArgs<'yp>> for UpdateYawPitch {

    fn update(&self, args: UpdateYawPitchArgs) {

        let UpdateYawPitchArgs { 
            yaw_pitch, mouse_pos, last_mouse_pos
        } = args;

        let mouse_delta: Vec2 = *mouse_pos - *last_mouse_pos;

        let yaw: &mut f32 = &mut yaw_pitch.yaw;
        let pitch: &mut f32 = &mut yaw_pitch.pitch;

        let delta = get_frame_time();

        *yaw += mouse_delta.x * delta * LOOK_SPEED;
        *pitch += mouse_delta.y * delta * -LOOK_SPEED;

        *pitch = if *pitch > 1.5 { 1.5 } else { *pitch };
        *pitch = if *pitch < -1.5 { -1.5 } else { *pitch };
    }
}


#[derive(Deref, DerefMut, Clone, Copy, Default)]
pub struct PlayerPos(pub Vec3);

/// # System
pub struct UpdatePlayerPosOnWASD;

pub struct UpdatePlayerPosArgs<'ppos, 'vecs> {
    pub player_pos: &'ppos mut PlayerPos,
    pub player_vecs: &'vecs PlayerVecs,
}

impl<'ppos, 'vecs> System<UpdatePlayerPosArgs<'ppos, 'vecs>> for UpdatePlayerPosOnWASD {
    fn update(&self, arg: UpdatePlayerPosArgs) {

        let UpdatePlayerPosArgs { 
            player_pos, player_vecs
        } = arg;

        let PlayerVecs { front, right, .. } = *player_vecs;

        if is_key_down(KeyCode::W) {
            player_pos.0 += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            player_pos.0 -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            player_pos.0 -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            player_pos.0 += right * MOVE_SPEED;
        }


    }
}

/// # System
pub struct UpdateVecsAfterYawPitch;

pub struct UpdateVecsArgs<'vecs, 'yp> {
    pub player_vecs: &'vecs mut PlayerVecs,
    pub yaw_pitch: &'yp YawPitch,
}

impl<'vecs, 'yp> System<UpdateVecsArgs<'vecs, 'yp>> for UpdateVecsAfterYawPitch {
    fn update(&self, args: UpdateVecsArgs) {
        let UpdateVecsArgs {
            player_vecs, yaw_pitch,
        } = args;

        let front = vec3(
            yaw_pitch.yaw.cos() * yaw_pitch.pitch.cos(),
            yaw_pitch.pitch.sin(),
            yaw_pitch.yaw.sin() * yaw_pitch.pitch.cos(),
        )
        .normalize();

        let right = front.cross(UP).normalize();
        let up = right.cross(front).normalize();

        *player_vecs = PlayerVecs { front, right, up };
    }
}


#[derive(Deref, DerefMut, Clone, Copy)]
pub struct LastMousePos(pub Vec2);

#[derive(Deref, DerefMut, Clone, Copy)]
pub struct MousePos(pub Vec2);


#[derive(Debug, Clone)]
pub struct PlayerVecs {
    pub front: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

impl PlayerVecs {

    pub fn new(yaw_pitch: &YawPitch) -> Self {

        let front = vec3(
            yaw_pitch.yaw.cos() * yaw_pitch.pitch.cos(),
            yaw_pitch.pitch.sin(),
            yaw_pitch.yaw.sin() * yaw_pitch.pitch.cos(),
        )
        .normalize();

        let right = front.cross(UP).normalize();
        let up = right.cross(front).normalize();

        Self { front, right, up }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct YawPitch {
    pub yaw: f32,
    pub pitch: f32,
}

impl YawPitch {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self { yaw, pitch }
    }
}

impl Default for YawPitch {
    fn default() -> Self {
        Self {
            yaw: 1.18,
            pitch: 0.0,
        }
    }
}