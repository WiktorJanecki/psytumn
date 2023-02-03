use bracket_noise::prelude::{FastNoise, NoiseType};
use glam::{UVec2, Vec2};
use hecs::With;
use rand::Rng;
use sdl2::{render::{TextureCreator}, video::WindowContext};
use sdl2_animation::{Keyframe, Animation};

use crate::{texturemanager::TextureManager, input::InputState, components, render::{Camera, Tilemap, Tile}};

pub struct Level1State{
    update_started: bool,
    texture_creator: TextureCreator<WindowContext>,
    texture_manager: TextureManager,
    world: hecs::World,
    camera: Camera,
    tilemap: Tilemap,
}

impl Level1State{
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Self{
        Self{
            update_started: false,
            texture_creator: canvas.texture_creator(),
            texture_manager: TextureManager::new(),
            world: hecs::World::new(),
            camera: Camera::new(),
            tilemap: Tilemap::new(50, 50, 32, 32),
        }
    }
}

pub fn update(state: &mut Level1State, dt: f32, input_state: &InputState){
    if !state.update_started{
        state.update_started = true;
        let idle_animation_player: Animation = vec![
            Keyframe{ x: 0, y: 0, width: 40, height: 40, duration: std::time::Duration::from_secs(1) },
            Keyframe{ x: 40, y: 0, width: 40, height: 40, duration: std::time::Duration::from_secs(1) }
        ];
        let idle_animation_snake: Animation = vec![
            Keyframe{ x: 0, y: 0, width: 32, height: 32, duration: std::time::Duration::from_secs(1) },
            Keyframe{ x: 32, y: 0, width: 32, height: 32, duration: std::time::Duration::from_secs(1) }
        ];
        let mut player_animation_state = components::Animation::default();
        let mut enemy_animation_state = components::Animation::default();
        player_animation_state.state.play(&idle_animation_player);
        enemy_animation_state.state.play(&idle_animation_snake);
        let _player = state.world.spawn((
            components::Player,
            components::Transform::default(),
            components::Sprite{ filename: "res/player.png", size: UVec2::new(40,40) },
            components::CameraTarget,
            components::PlayerController::default(),
            player_animation_state,
        ));
        let _enemy = state.world.spawn((
            components::Transform::with_position(640.0, 64.0),
            components::Sprite{ filename: "res/snake.png", size: UVec2::new(40,40) },
            components::GhostAI::default(),
            enemy_animation_state,
        ));
        // perlin generate water
        let mut rng = rand::thread_rng();
        let mut noise = FastNoise::seeded(rng.gen());
        noise.set_noise_type(NoiseType::Perlin);
        noise.set_frequency(0.09);
        let max_x = state.tilemap.values.len();
        let max_y = state.tilemap.values.get(0).unwrap().len();
        for x in 0..max_x{
            for y in 0..max_y{
                let value = noise.get_noise(x.clone() as f32,y.clone() as f32);
                if value > 0.2{
                    state.tilemap.set(x,y, Some(Tile{filename: "res/water.png"}));
                }
            }
        }
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

    // ghost ai
    let target_transform = (state.world.query::<(&components::Player, &components::Transform)>().iter().last().expect("Expect a player for ai to be targeted").1).1.clone();
    for (_id, (transform, ghost_ai)) in state.world.query_mut::<(&mut components::Transform, &mut components::GhostAI)>(){
        let difference = target_transform.position-transform.position;
        if difference.length() <= ghost_ai.radius{
            ghost_ai.velocity = difference.normalize()*ghost_ai.speed;
            transform.position += dt * ghost_ai.velocity;
        }
    }

    // camera follow
    for (_id, transform) in &mut state.world.query::<With<&components::Transform,&components::CameraTarget>>(){
        let smooth_value = 15.0;
        let offset = Vec2::new(-1280.0/2.0 + 40.0, -720.0/2.0 + 40.0); // TODO: MAKE IT NOT HARDCODED
        let target_position = transform.position + offset;
        let smoothed_position = state.camera.position.lerp(target_position, smooth_value * dt);
        state.camera.position = smoothed_position;
    }

    for (_id, animation_state) in state.world.query_mut::<&mut components::Animation>(){
        animation_state.state.update(std::time::Duration::from_secs_f32(dt));
    }
}

pub fn render(state: &mut Level1State, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {    
    canvas.set_draw_color(sdl2::pixels::Color::RGB(39,9,31));
    canvas.clear();
    let scale = 2;
    // render tilemap
    state.tilemap.values.iter().enumerate().for_each(|(x, xses)|{
        xses.iter().map(|f| f.as_ref()).enumerate().for_each(|(y, tile)|{
            if let Some(tile) = tile{
                let texture = state.texture_manager.texture(tile.filename, &state.texture_creator);
                let dst = sdl2::rect::Rect::new(
                    state.tilemap.position().x + state.camera.x() + (state.tilemap.tile_width * scale) as i32* x as i32,
                    state.tilemap.position().y + state.camera.y() + (state.tilemap.tile_height * scale) as i32 * y as i32,
                    state.tilemap.tile_width * scale,
                    state.tilemap.tile_height * scale
                );
                let _ = canvas.copy(texture, None, dst);
            }
            
        })
    });
    // render sprites
    for (id, (sprite, transform)) in &mut state.world.query::<(&components::Sprite, &components::Transform)>(){
        let dst = sdl2::rect::Rect::new(state.camera.x() + transform.position.x as i32, state.camera.y() + transform.position.y as i32,sprite.size.x * scale, sprite.size.y * scale);
        let src = state.world.entity(id).unwrap().get::<&components::Animation>().and_then(|f|Some(f.state.get_src()));
        let texture = state.texture_manager.texture(sprite.filename, &state.texture_creator);
        let _ = canvas.copy(texture, src, dst);
    }
    canvas.present();
}