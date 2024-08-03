use std::{f32::consts::PI, ops::Deref};

use macroquad::{math, models::Vertex, prelude::*};

mod systems;

mod world;

use world::chunk_mesh::*;
use world::*;

use systems::mouse::*;
use systems::yawpitch::*;
use systems::{grab::*, System};
use world::BlockState;

const MOVE_SPEED: f32 = 0.1;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let atlas: Texture2D = load_texture("assets/atlas.png").await.unwrap();
    atlas.set_filter(FilterMode::Nearest);

    let mut x = 0.0;
    let mut switch = false;
    let bounds = 8.0;

    let world_up = vec3(0.0, 1.0, 0.0);

    let mut yaw_pitch = YawPitch {
        yaw: 1.18,
        pitch: 0.0,
    };

    let mut front = vec3(
        yaw_pitch.yaw.cos() * yaw_pitch.pitch.cos(),
        yaw_pitch.pitch.sin(),
        yaw_pitch.yaw.sin() * yaw_pitch.pitch.cos(),
    )
    .normalize();

    let mut right = front.cross(world_up).normalize();
    let mut up = right.cross(front).normalize();

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let tab_press = TabPressSystem::init();
    let mut grab = GrabbedState(true);

    let update_yaw_pitch = UpdateYawPitch::init();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        tab_press.update(&mut grab);

        if is_key_down(KeyCode::W) {
            position += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            position -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            position -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            position += right * MOVE_SPEED;
        }

        let mouse_position: Vec2 = mouse_position().into();

        let yaw_pitch_state = YawPitchState {
            yaw_pitch: &mut yaw_pitch,
            mouse_pos: MousePos(mouse_position),
            last_mouse_pos: LastMousePos(last_mouse_position),
        };

        last_mouse_position = mouse_position;

        if grab.0 {
            update_yaw_pitch.update(yaw_pitch_state);

            front = vec3(
                yaw_pitch.yaw.cos() * yaw_pitch.pitch.cos(),
                yaw_pitch.pitch.sin(),
                yaw_pitch.yaw.sin() * yaw_pitch.pitch.cos(),
            )
            .normalize();

            right = front.cross(world_up).normalize();
            up = right.cross(front).normalize();

            x += if switch { 0.04 } else { -0.04 };
            if x >= bounds || x <= -bounds {
                switch = !switch;
            }
        }
        clear_background(Color {
            r: 0.3,
            g: 0.3,
            b: 0.5,
            a: 1.,
        });

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        let chunk: Chunk = Chunk::from_fn(|x, y, z| {
            if y < 3 {
                BlockState::STONE
            }
            else if y == 4 {
                BlockState::GRASS
            }
            else {
                BlockState::AIR
            }
        });

        let conn: Connected<Chunk> = Connected { 
            up: &Chunk::EMPTY,
            bo: &Chunk::EMPTY, 
            px: &Chunk::EMPTY, 
            pz: &Chunk::EMPTY, 
            nz: &Chunk::EMPTY, 
            nx: &Chunk::EMPTY, 
        };

        let mesh = chunk.build_chunk_mesh(
            ChunkPos { x: 0, y: 0, z: 0 }, 
            atlas.clone(), 
            &conn
        );

        draw_mesh(&mesh);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        draw_text(
            format!(
                "X: {:.2} Y: {:.2} Z: {:.2}",
                position.x, position.y, position.z
            )
            .as_str(),
            10.0,
            48.0 + 20.0,
            60.0,
            BLACK,
        );

        let fps = get_fps();
        draw_text(
            format!("FPS: {}", fps).as_str(),
            10.0,
            48.0 + 60.0,
            60.0,
            BLACK,
        );

        next_frame().await
    }
}
