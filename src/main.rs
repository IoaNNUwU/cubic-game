#![feature(const_fn_floating_point_arithmetic)]

use std::io::Read;
use std::net::TcpListener;

use macroquad::prelude::*;

mod systems;

mod world;

use world::render::*;
use world::*;

use systems::player::*;
use systems::{grab::*, System};
use world::BlockState;

fn conf() -> Conf {
    Conf {
        window_title: String::from("CubicGame"),
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

    let mut yaw_pitch = YawPitch::default();
    let mut front_right_up = FrontRightUpVecs::new(&yaw_pitch);

    let mut player_pos = PlayerPos(vec3(10., 10., 10.));

    let mut last_mouse_position: Vec2 = mouse_position().into();


    let update_grabbed_on_tab = TabPressSystem::new();
    let mut grabbed_state = GrabbedState::default();

    let mut chunk = Chunk::EMPTY;

    for x in 0..16 {
        for z in 0..16 {
            for y in 0..=3 {
                let rand = rand::gen_range(0, 5);
                match rand {
                    0 | 1 => *chunk.get_mut(x, y, z) = BlockState::DIRT,
                    2 | 3 => *chunk.get_mut(x, y, z) = BlockState::STONE,
                    _ => *chunk.get_mut(x, y, z) = BlockState::SAND,
                }
            }
        }
    }
    
    loop {
        if is_key_pressed(KeyCode::Escape) { break; }

        update_grabbed_on_tab.update(&mut grabbed_state);

        let mouse_position: Vec2 = mouse_position().into();

        let update_yaw_pitch_args = UpdateYawPitchArgs {
            yaw_pitch: &mut yaw_pitch,
            mouse_pos: MousePos(mouse_position),
            last_mouse_pos: LastMousePos(last_mouse_position),
        };
        last_mouse_position = mouse_position;

        if grabbed_state.0 {

            UpdateYawPitch.update(update_yaw_pitch_args);

            UpdatePlayerPosOnWASD.update(UpdatePlayerPosArgs {
                player_pos: &mut player_pos, player_vecs: &front_right_up,
            });

            UpdateVecsAfterYawPitch.update(UpdateVecsArgs { 
                player_vecs: &mut front_right_up, 
                yaw_pitch: &yaw_pitch,
            });
        }

        clear_background(Color {
            r: 0.3,
            g: 0.3,
            b: 0.5,
            a: 1.,
        });

        set_camera(&Camera3D {
            position: *player_pos,
            up: front_right_up.up,
            target: *player_pos + front_right_up.front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        let mut chunk_meshes: Vec<Mesh> = vec![];

        for ch_x in -16..16 {
            for ch_z in -16..16 {
                let chunk_pos = ChunkPos { x: ch_x, y: 0, z: ch_z };

                let chunk_model = build_chunk_model(
                    player_pos.0, front_right_up.front, chunk_pos, &chunk, &ConnectedChunks::EMPTY
                );
                chunk_meshes.extend(build_chunk_meshes(chunk_pos, chunk_model.clone(), Some(atlas.clone())))
            }
        }

        for chunk_mesh in &chunk_meshes {
            draw_mesh(&chunk_mesh);
        }

        // Back to screen space, render some text

        set_default_camera();
        draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);

        draw_text(
            format!(
                "X: {:.2} Y: {:.2} Z: {:.2}",
                player_pos.x, player_pos.y, player_pos.z
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

struct World([[[Chunk; 16]; 16]; 16]);