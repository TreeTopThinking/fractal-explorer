use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

const SHADER_PATH: &str = "shaders/shader.wesl";
const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 1000.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<FractalMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update_fractal_material)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FractalMaterial>>,
    window: Single<&Window>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(WIDTH, HEIGHT))),
        MeshMaterial2d(materials.add(FractalMaterial {
            mouse_pos: get_mouse_pos(&window),
            zoom: 3.0,
            center: vec2(2.0, 1.5),
        })),
    ));
}

fn update_fractal_material(
    mut materials: ResMut<Assets<FractalMaterial>>,
    scroll: Res<AccumulatedMouseScroll>,
    window: Single<&Window>,
    input: Res<ButtonInput<KeyCode>>,
) {
    const SPEED: f32 = 0.05;

    for (_, material) in materials.iter_mut() {
        material.mouse_pos = get_mouse_pos(&window);
        let zoom_factor = 2.0_f32.powf(-scroll.delta.y * SPEED);

        let scroll_delta = get_scroll_delta(&scroll);
        if scroll_delta != 0.0 {
            let old_zoom = material.zoom;
            let new_zoom = old_zoom * zoom_factor;

            material.center += material.mouse_pos * (new_zoom - old_zoom);
            material.zoom = new_zoom;
        }

        if input.pressed(KeyCode::KeyW) {
            material.center.y += SPEED * material.zoom;
        } else if input.pressed(KeyCode::KeyS) {
            material.center.y -= SPEED / zoom_factor * material.zoom;
        }

        if input.pressed(KeyCode::KeyA) {
            material.center.x += SPEED / zoom_factor * material.zoom;
        } else if input.pressed(KeyCode::KeyD) {
            material.center.x -= SPEED / zoom_factor * material.zoom;
        }
    }
}

fn get_mouse_pos(window: &Single<&Window>) -> Vec2 {
    let pos = window.cursor_position();

    let window_size = vec2(window.width(), window.height());
    let rect_size = vec2(WIDTH, HEIGHT);
    let rect_top_left = (window_size - rect_size) / 2.0;

    let pos = match pos {
        Some(pos) => pos,
        None => vec2(0.0, 0.0),
    };

    (pos - rect_top_left) / rect_size
}

fn get_scroll_delta(scroll: &Res<AccumulatedMouseScroll>) -> f32 {
    scroll.delta.y * 0.05
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct FractalMaterial {
    #[uniform(0)]
    mouse_pos: Vec2,
    #[uniform(0)]
    zoom: f32,
    #[uniform(0)]
    center: Vec2,
}

impl Material2d for FractalMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}
