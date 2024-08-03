use macroquad::prelude::*;

mod systems;

mod world;

use world::chunk_mesh::*;
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
    let mut player_vecs = PlayerVecs::new(&yaw_pitch);

    let mut player_pos = PlayerPos(vec3(10., 10., 10.));

    let mut last_mouse_position: Vec2 = mouse_position().into();


    let update_grabbed_on_tab = TabPressSystem::new();
    let mut grabbed_state = GrabbedState::default();

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
                player_pos: &mut player_pos, player_vecs: &player_vecs,
            });

            UpdateVecsAfterYawPitch.update(UpdateVecsArgs { 
                player_vecs: &mut player_vecs, 
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
            up: player_vecs.up,
            target: *player_pos + player_vecs.front,
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
