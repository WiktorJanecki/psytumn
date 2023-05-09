use glam::{UVec2};
use hecs::{With, Without};
use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{components, input::InputState, render::Camera, texturemanager::TextureManager, Level, systems::system_camera_follow};

pub struct MenuState {
    update_started: bool,
    texture_creator: TextureCreator<WindowContext>,
    texture_manager: TextureManager,
    camera: Camera,
    world: hecs::World,
    current_button: usize,
    button_pushed: bool,
    buttons_state: u32,
}

impl MenuState {
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        Self {
            update_started: false,
            texture_creator: canvas.texture_creator(),
            texture_manager: TextureManager::new(),
            camera: Camera::new(),
            world: hecs::World::new(),
            current_button: 1,
            button_pushed: false,
            buttons_state: 0,
        }
    }
}

pub fn update(state: &mut MenuState, dt: f32, input_state: &mut InputState, level: &mut Level) {
    if !state.update_started {
        state.update_started = true;

        spawn_button(&mut state.world, "res/quit.png", 0);
        spawn_button(&mut state.world, "res/spring.png", 1);
        spawn_button(&mut state.world, "res/summer.png", 2);
        spawn_button(&mut state.world, "res/autumn.png", 3);
        spawn_button(&mut state.world, "res/winter.png", 4);

        state.world.spawn((
            components::Sprite {
                filename: "res/bullet.png",
                size: UVec2::new(128 - 64, 128 - 64),
            },
            components::Transform::with_position(256.0 + 32.0, 32.0),
            components::CameraTarget,
        ));
    }

    for (_id, transform) in state
        .world
        .query_mut::<With<&mut components::Transform, &components::CameraTarget>>()
    {
        if input_state.movement.x > 0.0 && !state.button_pushed && state.current_button != 4 {
            state.button_pushed = true;
            transform.position.x += 256.0;
            state.current_button += 1;
        }
        if input_state.movement.x < 0.0 && !state.button_pushed && state.current_button != 0 {
            state.button_pushed = true;
            transform.position.x -= 256.0;
            state.current_button -= 1;
        }
        if input_state.movement.x == 0.0 {
            state.button_pushed = false;
        }
    }

    if (input_state.attack || input_state.dash) && is_button_available(state.buttons_state, state.current_button) {
        match state.current_button {
            0 => input_state.quit = true,
            1 => {}
            2 => {}
            3 => *level = Level::Level1,
            4 => {}
            _ => {}
        }
    }

    system_camera_follow(&state.world, &mut state.camera, dt);
}
pub fn render(state: &mut MenuState, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.clear();

    for (id, (sprite, transform, _)) in &mut state
        .world
        .query::<(&components::Sprite, &components::Transform, &Button)>()
    {
        let dst = sdl2::rect::Rect::new(
            state.camera.x() + transform.position.x as i32,
            state.camera.y() + transform.position.y as i32,
            sprite.size.x,
            sprite.size.y,
        );
        let src = state
            .world
            .entity(id)
            .unwrap()
            .get::<&components::Animation>()
            .and_then(|f| Some(f.state.get_src()));
        let texture = state
            .texture_manager
            .texture(sprite.filename, &state.texture_creator);
        let index = transform.position.x / 256.0;
        if is_button_available(state.buttons_state, index as usize){
            let _ = canvas.copy(texture, src, dst);
        }
    }
    for (id, (sprite, transform)) in &mut state
    .world
    .query::<Without<(&components::Sprite, &components::Transform), &Button>>()
    {
        let dst = sdl2::rect::Rect::new(
            state.camera.x() + transform.position.x as i32,
            state.camera.y() + transform.position.y as i32,
            sprite.size.x,
            sprite.size.y,
        );
        let src = state
            .world
            .entity(id)
            .unwrap()
            .get::<&components::Animation>()
            .and_then(|f| Some(f.state.get_src()));
        let texture = state
            .texture_manager
            .texture(sprite.filename, &state.texture_creator);
        let _ = canvas.copy(texture, src, dst);
    }

    canvas.present();
}

struct Button;
pub fn spawn_button(world: &mut hecs::World, texture: &'static str, index: usize) {
    let spacing = 256;
    world.spawn((
        components::Sprite {
            filename: texture,
            size: UVec2::new(128, 128),
        },
        components::Transform::with_position((spacing * index) as f32, 0.0),
        Button,
    ));
}

pub fn unblock_button(state: &mut MenuState, index: usize){
    state.buttons_state = state.buttons_state | (0b1<<index);
}

fn is_button_available(buttons_state: u32, index: usize) -> bool{
    return (buttons_state & (1 << index)) != 0
}