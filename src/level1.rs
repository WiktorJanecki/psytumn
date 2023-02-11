use bracket_noise::prelude::{FastNoise, NoiseType};
use glam::{UVec2, Vec2};
use hecs::With;
use rand::Rng;
use sdl2::{render::TextureCreator, video::WindowContext};
use sdl2_animation::{Animation, Keyframe};

use crate::{
    components,
    input::InputState,
    player_state,
    render::{Camera, Tile, Tilemap},
    texturemanager::TextureManager,
    Level,
};

pub struct Level1State<'a> {
    update_started: bool,
    texture_creator: TextureCreator<WindowContext>,
    texture_manager: TextureManager,
    world: hecs::World,
    camera: Camera,
    tilemap: Tilemap,
    points: u32,
    player_state_input: player_state::Input,
    enemy_spawner_timer: f32,
    music: sdl2::mixer::Music<'a>,
    sound_dash: sdl2::mixer::Chunk,
    sound_shoot: sdl2::mixer::Chunk,
    sound_crystal: sdl2::mixer::Chunk,
}

impl<'a> Level1State<'a> {
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        let music = sdl2::mixer::Music::from_file("res/music.wav").unwrap();
        let sound_dash = sdl2::mixer::Chunk::from_file("res/dash.wav").unwrap();
        let mut sound_shoot = sdl2::mixer::Chunk::from_file("res/shoot.wav").unwrap();
        sound_shoot.set_volume(50);
        let sound_crystal = sdl2::mixer::Chunk::from_file("res/crystal.wav").unwrap();
        Self {
            update_started: false,
            texture_creator: canvas.texture_creator(),
            texture_manager: TextureManager::new(),
            world: hecs::World::new(),
            camera: Camera::new(),
            tilemap: Tilemap::new(50, 50, 32, 32),
            points: 0,
            enemy_spawner_timer: 0.0,
            music,
            sound_dash,
            sound_shoot,
            sound_crystal,
            player_state_input: player_state::Input::Nothing,
        }
    }
}
pub fn update(state: &mut Level1State, dt: f32, input_state: &InputState, level: &mut Level) {
    puffin::profile_scope!("update");
    let mut rng = rand::thread_rng();
    if !state.update_started {
        state.update_started = true;

        let _ = state.music.play(-1);
        sdl2::mixer::Music::set_volume(10);

        let idle_animation_player: Animation = vec![
            Keyframe {
                x: 0,
                y: 0,
                width: 40,
                height: 40,
                duration: std::time::Duration::from_secs(1),
            },
            Keyframe {
                x: 40,
                y: 0,
                width: 40,
                height: 40,
                duration: std::time::Duration::from_secs(1),
            },
        ];
        let mut player_animation_state = components::Animation::default();
        player_animation_state.state.play(&idle_animation_player);
        let _player = state.world.spawn((
            components::Player::default(),
            components::Transform::default(),
            components::Sprite {
                filename: "res/player.png",
                size: UVec2::new(40, 40),
            },
            components::CameraTarget,
            components::PlayerController::default(),
            player_animation_state,
        ));
        // perlin generate water
        let mut noise = FastNoise::seeded(rng.gen());
        noise.set_noise_type(NoiseType::Perlin);
        noise.set_frequency(0.05);
        let max_x = state.tilemap.values.len();
        let max_y = state.tilemap.values.get(0).unwrap().len();
        for x in 0..max_x {
            for y in 0..max_y {
                let value = noise.get_noise(x.clone() as f32, y.clone() as f32);
                if value.abs() < 0.07 {
                    state.tilemap.set(
                        x,
                        y,
                        Some(Tile {
                            filename: "res/path.png",
                        }),
                    );
                }
                if value.abs() > 0.3{
                    state.tilemap.set(
                        x,
                        y,
                        Some(Tile {
                            filename: "res/grass.png",
                        }),
                    );
                }
            }
        }
        for _ in 0..32 {
            create_dash_crystal_on(
                state,
                rng.gen_range(-1600..1600),
                rng.gen_range(-1600..1600),
            );
        }
        for _ in 0..3 {
            create_point_crystal_on(
                state,
                rng.gen_range(-1600..1600),
                rng.gen_range(-1600..1600),
            );
        }
    }
    // Update
    // Reset player input state
    state.player_state_input = player_state::Input::Nothing;

    // Spawn enemies every second
    state.enemy_spawner_timer -= dt;
    if state.enemy_spawner_timer <= 0.0 {
        let enemy_spawn_cooldown = 1.0;
        state.enemy_spawner_timer = enemy_spawn_cooldown;
        create_enemy_on(
            &mut state.world,
            rng.gen_range(-1600..1600),
            rng.gen_range(-1600..1600),
        );
    }
    system_player_controller(
        &mut state.world,
        &mut state.player_state_input,
        &state.sound_shoot,
        &state.camera,
        input_state,
        dt,
    );
    system_ghost_ai(&mut state.world, dt);
    system_crystal(
        &mut state.world,
        &mut state.player_state_input,
        &mut state.points,
        &state.sound_crystal,
    );
    system_bullets(&mut state.world, dt);
    system_camera_follow(&state.world, &mut state.camera, dt);
    system_animation(&mut state.world, dt);
    if state.points >= 3 {
        *level = Level::Intro;
    }
    for (_id, player) in state.world.query_mut::<&mut components::Player>() {
        player_state::handle_state(
            &mut player.state_machine,
            &state.player_state_input,
            &state.sound_dash,
            dt,
        );
    }
}

pub fn render(state: &mut Level1State, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    puffin::profile_scope!("render");
    canvas.set_draw_color(sdl2::pixels::Color::RGB(39, 9, 31));
    canvas.clear();
    let scale = 2;
    // render tilemap
    state
        .tilemap
        .values
        .iter()
        .enumerate()
        .for_each(|(x, xses)| {
            xses.iter()
                .map(|f| f.as_ref())
                .enumerate()
                .for_each(|(y, tile)| {
                    if let Some(tile) = tile {
                        let texture = state
                            .texture_manager
                            .texture(tile.filename, &state.texture_creator);
                        let dst = sdl2::rect::Rect::new(
                            state.tilemap.position().x
                                + state.camera.x()
                                + (state.tilemap.tile_width * scale) as i32 * x as i32,
                            state.tilemap.position().y
                                + state.camera.y()
                                + (state.tilemap.tile_height * scale) as i32 * y as i32,
                            state.tilemap.tile_width * scale,
                            state.tilemap.tile_height * scale,
                        );
                        // render only if dst is in screen bounds + offset
                        let offset = 100;
                        if !dst.has_intersection(sdl2::rect::Rect::new(
                            -offset,
                            -offset,
                            1280 + offset as u32,
                            720 + offset as u32,
                        )) {
                            return;
                        }
                        let _ = canvas.copy(texture, None, dst);
                    }
                })
        });
    // render sprites
    for (id, (sprite, transform)) in &mut state
        .world
        .query::<(&components::Sprite, &components::Transform)>()
    {
        let dst = sdl2::rect::Rect::new(
            state.camera.x() + transform.position.x as i32,
            state.camera.y() + transform.position.y as i32,
            sprite.size.x * scale,
            sprite.size.y * scale,
        );
        // render only if dst is in screen bounds + offset
        let offset = 100;
        if !dst.has_intersection(sdl2::rect::Rect::new(
            -offset,
            -offset,
            1280 + offset as u32,
            720 + offset as u32,
        )) {
            continue;
        }
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

fn system_crystal(
    world: &mut hecs::World,
    player_state_input: &mut player_state::Input,
    points: &mut u32,
    sound_crystal: &sdl2::mixer::Chunk,
) {
    let mut optional_player_position = None;
    let mut optional_player_size = None;
    for (_id, (transform, sprite, _)) in &mut world.query::<(
        &components::Transform,
        &components::Sprite,
        &components::PlayerController,
    )>() {
        optional_player_position = Some(transform.position);
        optional_player_size = Some(sprite.size);
    }
    let mut crystals_to_delete = vec![];
    let mut should_be_stopped = false;
    if let (Some(target_position), Some(target_size)) =
        (optional_player_position, optional_player_size)
    {
        // if target exist
        // dash crystal
        for (crystal_id, (transform, sprite, _)) in &mut world.query::<(
            &components::Transform,
            &components::Sprite,
            &components::DashingCrystal,
        )>() {
            if sdl2::rect::Rect::new(
                target_position.x as i32,
                target_position.y as i32,
                target_size.x,
                target_size.y,
            )
            .has_intersection(sdl2::rect::Rect::new(
                transform.position.x as i32,
                transform.position.y as i32,
                sprite.size.x,
                sprite.size.y,
            )) {
                crystals_to_delete.push(crystal_id);
                should_be_stopped = true;
            }
        }

        if should_be_stopped {
            *player_state_input = player_state::Input::Crystal;
        }
        // Point crystal
        for (crystal_id, (transform, sprite, _)) in &mut world.query::<(
            &components::Transform,
            &components::Sprite,
            &components::PointCrystal,
        )>() {
            if sdl2::rect::Rect::new(
                target_position.x as i32,
                target_position.y as i32,
                target_size.x,
                target_size.y,
            )
            .has_intersection(sdl2::rect::Rect::new(
                transform.position.x as i32,
                transform.position.y as i32,
                sprite.size.x,
                sprite.size.y,
            )) {
                crystals_to_delete.push(crystal_id);
                let _ = sdl2::mixer::Channel::all().play(sound_crystal, 0);
                *points += 1;
            }
        }
    }
    for crystal in crystals_to_delete.iter() {
        let _ = world.despawn(*crystal);
    }
}

fn system_ghost_ai(world: &mut hecs::World, dt: f32) {
    let mut optional_player_position = None;
    let mut optional_player_size = None;
    for (_id, (transform, sprite, _)) in &mut world.query::<(
        &components::Transform,
        &components::Sprite,
        &components::Player,
    )>() {
        optional_player_position = Some(transform.position);
        optional_player_size = Some(sprite.size);
    }
    if let (Some(target_pos), Some(target_size)) = (optional_player_position, optional_player_size)
    {
        // ghost move
        for (_id, (transform, ghost_ai)) in
            world.query_mut::<(&mut components::Transform, &mut components::GhostAI)>()
        {
            let difference = target_pos - transform.position;
            if difference.length() <= ghost_ai.radius {
                ghost_ai.velocity = difference.normalize() * ghost_ai.speed;
                transform.position += dt * ghost_ai.velocity;
            }
        }
        // Player death
        for (_id, (transform, sprite, _)) in &mut world.query::<(
            &components::Transform,
            &components::Sprite,
            &components::GhostAI,
        )>() {
            if sdl2::rect::Rect::new(
                target_pos.x as i32,
                target_pos.y as i32,
                target_size.x,
                target_size.y,
            )
            .has_intersection(sdl2::rect::Rect::new(
                transform.position.x as i32,
                transform.position.y as i32,
                sprite.size.x,
                sprite.size.y,
            )) {
                panic!("GAME OVER");
            }
        }
    }
}

fn system_animation(world: &mut hecs::World, dt: f32) {
    for (_id, animation_state) in world.query_mut::<&mut components::Animation>() {
        animation_state
            .state
            .update(std::time::Duration::from_secs_f32(dt));
    }
}

fn system_bullets(world: &mut hecs::World, dt: f32) {
    // Update bullet position
    for (_id, (transform, bullet)) in
        world.query_mut::<(&mut components::Transform, &components::Bullet)>()
    {
        transform.position += bullet.velocity * dt;
    }
    let mut bullets_ids_to_kill = vec![];
    let mut enemies_ids_to_kill = vec![];
    for (bullet_id, (transform, sprite, _)) in &mut world.query::<(
        &components::Transform,
        &components::Sprite,
        &components::Bullet,
    )>() {
        let bullet_rect = sdl2::rect::Rect::new(
            transform.position.x as i32,
            transform.position.y as i32,
            sprite.size.x,
            sprite.size.y,
        );
        for (enemy_id, (enemy_transform, enemy_sprite, _)) in &mut world.query::<(
            &components::Transform,
            &components::Sprite,
            &components::GhostAI,
        )>() {
            let enemy_rect = sdl2::rect::Rect::new(
                enemy_transform.position.x as i32,
                enemy_transform.position.y as i32,
                enemy_sprite.size.x,
                enemy_sprite.size.y,
            );
            if bullet_rect.has_intersection(enemy_rect) {
                bullets_ids_to_kill.push(bullet_id);
                enemies_ids_to_kill.push(enemy_id);
                break;
            }
        }
    }
    for bullet in bullets_ids_to_kill.iter() {
        let _ = world.despawn(*bullet);
    }

    for enemy in enemies_ids_to_kill.iter() {
        let _ = world.despawn(*enemy);
    }
}

fn system_camera_follow(world: &hecs::World, camera: &mut Camera, dt: f32) {
    for (_id, transform) in
        &mut world.query::<With<&components::Transform, &components::CameraTarget>>()
    {
        let smooth_value = 15.0;
        let offset = Vec2::new(-1280.0 / 2.0 + 40.0, -720.0 / 2.0 + 40.0); // TODO: MAKE IT NOT HARDCODED
        let target_position = transform.position + offset;
        let smoothed_position = camera.position.lerp(target_position, smooth_value * dt);
        camera.position = smoothed_position;
    }
}

fn system_player_controller(
    world: &mut hecs::World,
    player_state_input: &mut player_state::Input,
    sound_shoot: &sdl2::mixer::Chunk,
    camera: &Camera,
    input_state: &InputState,
    dt: f32,
) {
    let mut bullets_to_create = vec![];
    for (_id, (transform, controller, player)) in world.query_mut::<(
        &mut components::Transform,
        &mut components::PlayerController,
        &mut components::Player,
    )>() {
        let friction = 50.0 * 64.0;
        let max_vel = 12.0 * 64.0; // GREAT VALUES 64 is one tile
        let accel = 130.0 * 64.0;

        controller.acceleration = input_state.movement * accel; // apply movement direction
        controller.velocity += dt * controller.acceleration; // apply acceleration
        if controller.velocity.length() != 0.0 {
            controller.velocity *= 1.0 - (friction * dt) / controller.velocity.length();
            // apply friction
        }
        controller.velocity = controller.velocity.clamp_length_max(max_vel); // clamp velocity
        if controller.velocity.length() < 5.0{
            controller.velocity = Vec2::ZERO;
        }
        controller.dashing_time_left -= dt;
        controller.dashing_timer -= dt;
        controller.attack_timer -= dt;
        match player.state_machine.state {
            player_state::State::Moving => {
                if input_state.dash {
                    *player_state_input = player_state::Input::Dash;
                } else {
                    if input_state.movement != Vec2::ZERO {
                        *player_state_input = player_state::Input::Move;
                    }
                    transform.position += controller.velocity * dt; // apply velocity
                }
            }
            player_state::State::Dashing => {
                transform.position += input_state.movement.normalize_or_zero() * dt * max_vel * 3.0;
            }
            player_state::State::Stopped => {
                controller.velocity = Vec2::ZERO;
                if input_state.dash {
                    *player_state_input = player_state::Input::Dash;
                }
            }
            player_state::State::Idle => {
                if input_state.movement != Vec2::ZERO {
                    *player_state_input = player_state::Input::Move;
                }
                transform.position += controller.velocity * dt; // apply velocity
            }
        }

        if input_state.attack && controller.attack_timer <= 0.0 {
            let attack_cooldown = 1.0;
            controller.attack_timer = attack_cooldown;
            let direction = ((input_state.mouse_pos + camera.position) - transform.position)
                .normalize_or_zero();
            let _ = sdl2::mixer::Channel::all().play(sound_shoot, 0);
            bullets_to_create.push((transform.position, direction));
        }
    }
    for (pos, dir) in bullets_to_create {
        create_bullet(world, pos, dir);
    }
}

fn create_dash_crystal_on(state: &mut Level1State, x: i32, y: i32) {
    let mut crystal_animation_state = components::Animation::default();
    crystal_animation_state.state.play(&vec![
        Keyframe {
            x: 0,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(1.0),
        },
        Keyframe {
            x: 32,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(0.1),
        },
        Keyframe {
            x: 64,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(0.1),
        },
        Keyframe {
            x: 96,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(0.1),
        },
    ]);
    state.world.spawn((
        components::Transform {
            position: Vec2::new(x as f32, y as f32),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        },
        components::Sprite {
            filename: "res/crystal.png",
            size: UVec2::new(40, 40),
        },
        components::DashingCrystal,
        crystal_animation_state,
    ));
}

fn create_point_crystal_on(state: &mut Level1State, x: i32, y: i32) {
    let mut crystal_animation_state = components::Animation::default();
    crystal_animation_state.state.play(&vec![
        Keyframe {
            x: 0,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(1.0),
        },
        Keyframe {
            x: 32,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(0.1),
        },
        Keyframe {
            x: 64,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(0.1),
        },
        Keyframe {
            x: 96,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(0.1),
        },
    ]);
    state.world.spawn((
        components::Transform {
            position: Vec2::new(x as f32, y as f32),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        },
        components::Sprite {
            filename: "res/crystal_point.png",
            size: UVec2::new(40, 40),
        },
        components::PointCrystal,
        crystal_animation_state,
    ));
}

fn create_enemy_on(world: &mut hecs::World, x: i32, y: i32) {
    let idle_animation_snake: Animation = vec![
        Keyframe {
            x: 0,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(0.5),
        },
        Keyframe {
            x: 32,
            y: 0,
            width: 32,
            height: 32,
            duration: std::time::Duration::from_secs_f32(0.5),
        },
    ];
    let mut enemy_animation_state = components::Animation::default();
    enemy_animation_state.state.play(&idle_animation_snake);
    world.spawn((
        components::Transform::with_position(x as f32, y as f32),
        components::Sprite {
            filename: "res/snake.png",
            size: UVec2::new(40, 40),
        },
        components::GhostAI::default(),
        enemy_animation_state,
    ));
}

fn create_bullet(world: &mut hecs::World, position: Vec2, direction: Vec2) {
    let speed = 64.0 * 15.0;
    world.spawn((
        components::Transform::with_position(position.x, position.y),
        components::Sprite {
            filename: "res/bullet.png",
            size: UVec2::new(16, 16),
        },
        components::Bullet {
            velocity: direction * speed,
        },
    ));
}
