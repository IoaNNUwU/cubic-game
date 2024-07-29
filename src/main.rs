use std::{f32::consts::PI, ops::Deref};

use macroquad::{math, models::Vertex, prelude::*};

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.1;

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
    let mut yaw: f32 = 1.18;
    let mut pitch: f32 = 0.0;

    let mut front = vec3(
        yaw.cos() * pitch.cos(),
        pitch.sin(),
        yaw.sin() * pitch.cos(),
    )
    .normalize();

    let mut right = front.cross(world_up).normalize();
    let mut up = right.cross(front).normalize();

    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_position: Vec2 = mouse_position().into();

    let mut grabbed = true;
    set_cursor_grab(grabbed);
    show_mouse(false);

    loop {
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

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
        let mouse_delta = mouse_position - last_mouse_position;

        last_mouse_position = mouse_position;

        if grabbed {
            yaw += mouse_delta.x * delta * LOOK_SPEED;
            pitch += mouse_delta.y * delta * -LOOK_SPEED;

            pitch = if pitch > 1.5 { 1.5 } else { pitch };
            pitch = if pitch < -1.5 { -1.5 } else { pitch };

            front = vec3(
                yaw.cos() * pitch.cos(),
                pitch.sin(),
                yaw.sin() * pitch.cos(),
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

        // Going 3d!

        set_camera(&Camera3D {
            position: position,
            up: up,
            target: position + front,
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        let dirt: UvTexture = UvTexture::from_n(0);
        let grass_side = UvTexture::from_n(1);
        let grass_top = UvTexture::from_n(2);
        let stone = UvTexture::from_n(3);

        let cube_textures = CubeTextures {
            top: grass_top, side: grass_side, bottom: dirt
        };

        for x in -200..200 {
            for z in -200..200 {
                // draw_cube(vec3(x as f32, 0., z as f32), Vec3::ONE, Some(&atlas), WHITE);
                
                let cube_pos = vec3(x as f32, 0., z as f32);
                let player_pos = position;

                let cube_view_vec: Vec3 = cube_pos - player_pos;

                let angle = cube_view_vec.angle_between(front);

                if angle < 40.0f32.to_radians() {

                    let distance = player_pos.distance(cube_pos);
                    
                    if distance > 50. && (x % 2 == 0 || z % 2 == 0) {
                        draw_cube(cube_pos, vec3(2.0, 2.0, 2.0), Some(&atlas), WHITE);
                    }
                    else {
                        let mesh = make_cube(cube_textures, world_pos(x, 0, z), atlas.clone());
                        draw_mesh(&mesh);
                    }
                };
            }
        }

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

#[derive(Clone, Copy)]
struct UvTexture(Vec2);

impl Deref for UvTexture {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UvTexture {

    fn from_n(n: usize) -> UvTexture {
        UvTexture::new(vec2(0., 1. / 100. * n as f32))
    }
    fn new(inner: Vec2) -> UvTexture {
        Self(inner)
    }

    fn up_left(&self) -> Vec2 {
        self.0
    }
    fn up_right(&self) -> Vec2 {
        self.0 + vec2(1., 0.)
    }
    fn low_left(&self) -> Vec2 {
        self.0 + vec2(0., 0.01)
    }
    fn low_right(&self) -> Vec2 {
        self.0 + vec2(1., 0.01)
    }
}

fn vertex(pos: Vec3, uv: Vec2) -> Vertex {
    Vertex {
        position: pos,
        uv,
        color: WHITE,
    }
}

#[derive(Clone, Copy)]
struct CubeTextures {
    top: UvTexture,
    side: UvTexture,
    bottom: UvTexture,
}

struct WorldPos {
    x: isize,
    y: isize,
    z: isize,
}

fn world_pos(x: isize, y: isize, z: isize) -> WorldPos {
    WorldPos {
        x, y, z
    }
}

fn make_cube(textures: CubeTextures, pos: WorldPos, atlas: Texture2D) -> Mesh {

    let CubeTextures { top, side, bottom } = textures;

    let WorldPos { x, y, z } = pos;

    let (x, y, z) = (x as f32, y as f32, z as f32);

    Mesh {
        vertices: vec![
            vertex(vec3(0. + x, 0. + y, 0. + z), side.low_left()) ,
            vertex(vec3(1. + x, 0. + y, 0. + z), side.low_right()),
            vertex(vec3(1. + x, 1. + y, 0. + z), side.up_right()) ,

            vertex(vec3(0. + x, 0. + y, 0. + z), side.low_left()) ,
            vertex(vec3(0. + x, 1. + y, 0. + z), side.up_left())  ,
            vertex(vec3(1. + x, 1. + y, 0. + z), side.up_right()) ,

            vertex(vec3(0. + x, 0. + y, 0. + z), side.low_right()),
            vertex(vec3(0. + x, 1. + y, 0. + z), side.up_right()) ,
            vertex(vec3(0. + x, 1. + y, 1. + z), side.up_left())  ,

            vertex(vec3(0. + x, 0. + y, 0. + z), side.low_right()),
            vertex(vec3(0. + x, 0. + y, 1. + z), side.low_left()) ,
            vertex(vec3(0. + x, 1. + y, 1. + z), side.up_left())  ,

            vertex(vec3(0. + x, 0. + y, 1. + z), side.low_right()) ,
            vertex(vec3(1. + x, 0. + y, 1. + z), side.low_left()),
            vertex(vec3(1. + x, 1. + y, 1. + z), side.up_left()) ,

            vertex(vec3(0. + x, 0. + y, 1. + z), side.low_right()) ,
            vertex(vec3(0. + x, 1. + y, 1. + z), side.up_right())  ,
            vertex(vec3(1. + x, 1. + y, 1. + z), side.up_left()) ,

            vertex(vec3(1. + x, 0. + y, 0. + z), side.low_right()),
            vertex(vec3(1. + x, 1. + y, 0. + z), side.up_right()) ,
            vertex(vec3(1. + x, 1. + y, 1. + z), side.up_left())  ,

            vertex(vec3(1. + x, 0. + y, 0. + z), side.low_right()),
            vertex(vec3(1. + x, 0. + y, 1. + z), side.low_left()) ,
            vertex(vec3(1. + x, 1. + y, 1. + z), side.up_left())  ,

            // UP
            vertex(vec3(0. + x, 1. + y, 0. + z), top.low_left()),
            vertex(vec3(1. + x, 1. + y, 0. + z), top.low_right()),
            vertex(vec3(1. + x, 1. + y, 1. + z), top.up_right()),
            
            vertex(vec3(0. + x, 1. + y, 0. + z), top.low_left()),
            vertex(vec3(0. + x, 1. + y, 1. + z), top.low_right()),
            vertex(vec3(1. + x, 1. + y, 1. + z), top.up_right()),

            // LOW
            vertex(vec3(0. + x, 0. + y, 0. + z), bottom.low_left()),
            vertex(vec3(1. + x, 0. + y, 0. + z), bottom.low_right()),
            vertex(vec3(1. + x, 0. + y, 1. + z), bottom.up_right()),
            
            vertex(vec3(0. + x, 0. + y, 0. + z), bottom.low_left()),
            vertex(vec3(0. + x, 0. + y, 1. + z), bottom.low_right()),
            vertex(vec3(1. + x, 0. + y, 1. + z), bottom.up_right()),
        ],
        indices: vec![
            0, 1, 2,
            3, 4, 5,

            6, 7, 8,
            9, 10, 11,

            12, 13, 14,
            15, 16, 17,

            18, 19, 20,
            21, 22, 23,

            24, 25, 26,
            27, 28, 29,

            30, 31, 32,
            33, 34, 35,
        ],
        texture: Some(atlas),
    }
}