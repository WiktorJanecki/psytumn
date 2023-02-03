use glam::UVec2;
use sdl2::{render::{TextureCreator}, video::WindowContext};
use sdl2_animation::{Keyframe, Animation};

use crate::{texturemanager::{TextureManager}, input::InputState, components};

pub struct Level1State{
    update_started: bool,
    texture_creator: TextureCreator<WindowContext>,
    texture_manager: TextureManager,
    world: hecs::World,
}

impl Level1State{
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Self{
        Self{
            update_started: false,
            texture_creator: canvas.texture_creator(),
            texture_manager: TextureManager::new(),
            world: hecs::World::new(),
        }
    }
}

pub fn update(state: &mut Level1State, dt: f32, input_state: &InputState){
    if !state.update_started{
        state.update_started = true;
        let idle_animation: Animation = vec![
            Keyframe{ x: 0, y: 0, width: 40, height: 40, duration: std::time::Duration::from_secs(1) },
            Keyframe{ x: 40, y: 0, width: 40, height: 40, duration: std::time::Duration::from_secs(1) }
        ];
        let mut player_animation_state = components::Animation::default();
        let mut enemy_animation_state = components::Animation::default();
        player_animation_state.state.play(&idle_animation);
        enemy_animation_state.state.play(&idle_animation);
        let _player = state.world.spawn((
            components::Player,
            components::Transform::default(),
            components::Sprite{ filename: "res/player.png", size: UVec2::new(40,40) },
            components::PlayerController::default(),
            player_animation_state,
        ));
        let _enemy = state.world.spawn((
            components::Transform::with_position(640.0, 64.0),
            components::Sprite{ filename: "res/player.png", size: UVec2::new(40,40) },
            enemy_animation_state,
        ));
    }
    // Update

    for (_id, (transform, controller)) in state.world.query_mut::<(&mut components::Transform, &mut components::PlayerController)>(){
        let friction = 50.0 * 64.0;
        let max_vel = 12.0 * 64.0;    // GREAT VALUES 64 is one tile
        let accel = 130.0 * 64.0;

        controller.acceleration = input_state.movement * accel; // apply movement direction
        controller.velocity += dt * controller.acceleration; // apply acceleration
        if controller.velocity.length() != 0.0{
            controller.velocity *= 1.0 - (friction * dt ) / controller.velocity.length(); // apply friction
        }
        controller.velocity = controller.velocity.clamp_length_max(max_vel); // clamp velocity
        transform.position += controller.velocity * dt; // apply velocity
    }

    for (_id, animation_state) in state.world.query_mut::<&mut components::Animation>(){
        animation_state.state.update(std::time::Duration::from_secs_f32(dt));
    }
}

pub fn render(state: &mut Level1State, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {    
    canvas.set_draw_color(sdl2::pixels::Color::RGB(39,9,31));
    canvas.clear();
    let scale = 3;
    for (id, (sprite, transform)) in &mut state.world.query::<(&components::Sprite, &components::Transform)>(){
        let dst = sdl2::rect::Rect::new(transform.position.x as i32, transform.position.y as i32,sprite.size.x * scale, sprite.size.y * scale);
        let src = state.world.entity(id).unwrap().get::<&components::Animation>().and_then(|f|Some(f.state.get_src()));
        let texture = state.texture_manager.texture(sprite.filename, &state.texture_creator);
        let _ = canvas.copy(texture, src, dst);
    }
    canvas.present();
}