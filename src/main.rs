use bevy::{
    input::mouse::{AccumulatedMouseScroll, MouseMotion},
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
    mouse: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>,
    window: Single<&Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const ZOOM_SPEED: f32 = 3.0;
    const MOUSE_MOVE_SPEED: f32 = 0.1;
    const KEYBOARD_MOVE_SPEED: f32 = 0.4;

    for (_, material) in materials.iter_mut() {
        material.mouse_pos = get_mouse_pos(&window);
        let zoom_factor = 2.0_f32.powf(-scroll.delta.y * ZOOM_SPEED * time.delta_secs());

        let scroll_delta = get_scroll_delta(&scroll);
        if scroll_delta != 0.0 {
            let old_zoom = material.zoom;
            let new_zoom = old_zoom * zoom_factor;

            material.center += material.mouse_pos * (new_zoom - old_zoom);
            material.zoom = new_zoom;
        }

        if mouse.pressed(MouseButton::Left) {
            for event in mouse_motion.read() {
                material.center +=
                    event.delta * material.zoom * time.delta_secs() * MOUSE_MOVE_SPEED;
            }
        }

        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            material.center.y += KEYBOARD_MOVE_SPEED * time.delta_secs() * material.zoom;
        } else if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            material.center.y -= KEYBOARD_MOVE_SPEED * time.delta_secs() * material.zoom;
        }

        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            material.center.x += KEYBOARD_MOVE_SPEED * time.delta_secs() * material.zoom;
        } else if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            material.center.x -= KEYBOARD_MOVE_SPEED * time.delta_secs() * material.zoom;
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
