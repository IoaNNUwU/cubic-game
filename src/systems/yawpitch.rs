use super::*;
use macroquad::prelude::*;
use super::mouse::*;

const LOOK_SPEED: f32 = 0.1;

pub struct YawPitch {
    pub yaw: f32,
    pub pitch: f32,
}

pub struct UpdateYawPitch(());

pub struct YawPitchState<'yp> {
    pub yaw_pitch: &'yp mut YawPitch,
    pub mouse_pos: MousePos,
    pub last_mouse_pos: LastMousePos,
}

impl<'yp> System<YawPitchState<'yp>> for UpdateYawPitch {

    fn init() -> Self {
        Self(())
    }

    fn update(&self, state: YawPitchState) {

        let YawPitchState { yaw_pitch, mouse_pos, last_mouse_pos } = state;

        let mouse_delta = mouse_pos.0 - last_mouse_pos.0;

        let yaw: &mut f32 = &mut yaw_pitch.yaw;
        let pitch: &mut f32 = &mut yaw_pitch.pitch;

        let delta = get_frame_time();

        *yaw += mouse_delta.x * delta * LOOK_SPEED;
        *pitch += mouse_delta.y * delta * -LOOK_SPEED;

        *pitch = if *pitch > 1.5 { 1.5 } else { *pitch };
        *pitch = if *pitch < -1.5 { -1.5 } else { *pitch };
    }
}