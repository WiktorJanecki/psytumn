use bracket_noise::prelude::{FastNoise, NoiseType};
use glam::{UVec2, Vec2};
use hecs::With;
use rand::{rngs::ThreadRng, Rng};
use sdl2::{render::TextureCreator, video::WindowContext};
use sdl2_animation::{Animation, Keyframe};

use crate::{
    components::{self, BulletType},
    input::InputState,
    player_state,
    render::{Camera, Tile, Tilemap},
    texturemanager::TextureManager,
    Level,
};

const MOB_LIMIT: u32 = 20;
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
    player_death: bool,
    mob_count: u32,
    particles_state: sdl2_particles::ParticlesState,
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
            player_death: false,
            mob_count: 0,
            particles_state: sdl2_particles::ParticlesState::init(100),
        }
    }
}
pub fn update(
    state: &mut Level1State,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    dt: f32,
    input_state: &InputState,
    level: &mut Level,
) {
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
                if value.abs() > 0.3 {
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
        let mut dash_crystal_count = 0;
        while dash_crystal_count <= 50 {
            let mut position = Vec2::new(
                rng.gen_range(-1600.0..1600.0),
                rng.gen_range(-1600.0..1600.0),
            );
            let mut should_create_next_one = true;
            while should_create_next_one {
                dash_crystal_count += 1;
                let random_dir = match rng.gen_range(1..=8) {
                    1 => Vec2::new(1.0, 0.0),            //
                    2 => Vec2::new(1.0, 1.0),            //
                    3 => Vec2::new(0.0, 1.0),            //
                    4 => Vec2::new(-1.0, 1.0),           //
                    5 => Vec2::new(-1.0, 0.0),           //
                    6 => Vec2::new(-1.0, -1.0),          //
                    7 => Vec2::new(0.0, -1.0),           //
                    8 => Vec2::new(1.0, -1.0),           //
                    _ => panic!("WTF random is broken"), //
                }
                .normalize();
                let distance = 400.0;
                position += random_dir * distance;
                create_dash_crystal_on(state, position.x as i32, position.y as i32);

                should_create_next_one = rng.gen_bool(0.50);
            }
        }
        {
            // GENERATE CRYSTALS IN CORNERS
            let map_size_x = 49;
            let map_size_y = 49;
            let mut max_x = 5;
            let mut max_y = 5;
            loop {
                let try_x = rng.gen_range(0..=max_x);
                let try_y = rng.gen_range(0..=max_y);
                if let Some(Tile { filename }) = state.tilemap.get(try_x, try_y) {
                    if filename == &"res/path.png" {
                        create_point_crystal_on(
                            state,
                            try_x as i32 * state.tilemap.tile_width as i32 * 2
                                + state.tilemap.position().x,
                            try_y as i32 * state.tilemap.tile_height as i32 * 2
                                + state.tilemap.position().y,
                        );
                        break;
                    }
                }
                max_x += 1;
                max_x = max_x.clamp(0, map_size_x);
                max_y += 1;
                max_y = max_y.clamp(0, map_size_y);
            }
            let mut max_x = 5;
            let mut max_y = 5;
            loop {
                let try_x = rng.gen_range(0..=max_x);
                let try_y = rng.gen_range(0..=max_y);
                if let Some(Tile { filename }) = state.tilemap.get(map_size_x - try_x, try_y) {
                    if filename == &"res/path.png" {
                        create_point_crystal_on(
                            state,
                            (map_size_x - try_x) as i32 * state.tilemap.tile_width as i32 * 2
                                + state.tilemap.position().x,
                            try_y as i32 * state.tilemap.tile_height as i32 * 2
                                + state.tilemap.position().y,
                        );
                        break;
                    }
                }
                max_x += 1;
                max_x = max_x.clamp(0, map_size_x);
                max_y += 1;
                max_y = max_y.clamp(0, map_size_y);
            }
            let mut max_x = 5;
            let mut max_y = 5;
            loop {
                let try_x = rng.gen_range(0..=max_x);
                let try_y = rng.gen_range(0..=max_y);
                if let Some(Tile { filename }) =
                    state.tilemap.get(map_size_x - try_x, map_size_y - try_y)
                {
                    if filename == &"res/path.png" {
                        create_point_crystal_on(
                            state,
                            (map_size_x - try_x) as i32 * state.tilemap.tile_width as i32 * 2
                                + state.tilemap.position().x,
                            (map_size_y - try_y) as i32 * state.tilemap.tile_height as i32 * 2
                                + state.tilemap.position().y,
                        );
                        break;
                    }
                }
                max_x += 1;
                max_x = max_x.clamp(0, map_size_x);
                max_y += 1;
                max_y = max_y.clamp(0, map_size_y);
            }
        }
        for _ in 0..MOB_LIMIT / 4 {
            state.mob_count += 1;
            create_enemy_on(
                &mut state.world,
                rng.gen_range(-1600..1600),
                rng.gen_range(-1600..1600),
                &mut rng,
            );
        }
    }
    // Update
    // Reset player input state
    state.player_state_input = player_state::Input::Nothing;

    // Spawn enemies every second
    state.enemy_spawner_timer -= dt;
    if state.enemy_spawner_timer <= 0.0 && state.mob_count <= MOB_LIMIT {
        state.mob_count += 1;
        let enemy_spawn_cooldown = 1.0;
        state.enemy_spawner_timer = enemy_spawn_cooldown;
        create_enemy_on(
            &mut state.world,
            rng.gen_range(-1600..1600),
            rng.gen_range(-1600..1600),
            &mut rng,
        );
    }
    state
        .particles_state
        .update(std::time::Duration::from_secs_f32(dt));
    system_player_controller(
        &mut state.world,
        &mut state.player_state_input,
        &mut state.particles_state,
        &state.sound_shoot,
        &state.camera,
        input_state,
        &mut rng,
        dt,
    );
    system_ghost_ai(&mut state.world, &mut state.player_death, dt);
    system_shooting_enemies(state, dt);
    system_orbit_ai(state, dt);
    system_crystal(
        &mut state.world,
        &mut state.player_state_input,
        &mut state.particles_state,
        &mut state.points,
        &state.sound_crystal,
        &mut rng,
    );
    system_bullets(
        &mut state.world,
        &mut state.player_death,
        &mut state.mob_count,
        dt,
    );
    system_camera_follow(&state.world, &mut state.camera, dt);
    system_animation(&mut state.world, dt);
    if state.points >= 3 {
        *level = Level::Intro;
    }
    if state.player_death {
        *state = Level1State::new(canvas);
        *level = Level::Menu;
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
    // render particles
    state
        .particles_state
        .render_with_offset(state.camera.x(), state.camera.y(), canvas);
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

fn player_damage(world: &mut hecs::World, player_death: &mut bool) {
    let cooldown = 0.5;
    for (_, player) in world.query_mut::<&mut components::Player>() {
        if player.invincibility_timer <= 0.0 && false {
            player.lives -= 1;
            player.invincibility_timer = cooldown;
            println!("Player took damage! Remaining lives: {}/3", &player.lives);
            if player.lives <= 0 {
                *player_death = true;
            }
        }
    }
}

fn system_shooting_enemies(state: &mut Level1State, dt: f32) {
    let mut optional_player_position = None;
    let mut bullets_to_create = vec![];
    for (_id, (transform, _)) in &mut state
        .world
        .query::<(&components::Transform, &components::Player)>()
    {
        optional_player_position = Some(transform.position);
    }
    for (_id, (transform, shooting)) in state
        .world
        .query_mut::<(&components::Transform, &mut components::ShootingEnemy)>()
    {
        shooting.timer -= dt;
        if let Some(target_position) = optional_player_position {
            let direction = target_position - transform.position;
            if shooting.timer <= 0.0 && direction.length() <= shooting.range {
                shooting.timer = shooting.cooldown;
                bullets_to_create.push((
                    transform.position,
                    direction.normalize(),
                    BulletType::FromEnemy,
                ));
            }
        }
    }
    for bullet in bullets_to_create {
        create_bullet(&mut state.world, bullet.0, bullet.1, bullet.2);
    }
}

fn system_crystal(
    world: &mut hecs::World,
    player_state_input: &mut player_state::Input,
    particles_state: &mut sdl2_particles::ParticlesState,
    points: &mut u32,
    sound_crystal: &sdl2::mixer::Chunk,
    rng: &mut ThreadRng,
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
                for _ in 0..60 {
                    let particle_type = sdl2_particles::ParticleTypeBuilder::new(
                        rng.gen_range(4..16),
                        rng.gen_range(4..16),
                        std::time::Duration::from_millis(rng.gen_range(200..300)),
                    )
                    .with_color(sdl2::pixels::Color::RGB(
                        rng.gen_range(71..111),
                        rng.gen_range(5..45),
                        rng.gen_range(20..60),
                    )) // 91 25 40
                    .with_effect(sdl2_particles::ParticleEffect::LinearRotation {
                        angular_velocity: 30.0,
                    })
                    .with_effect(sdl2_particles::ParticleEffect::FadeOut {
                        delay: std::time::Duration::from_millis(150),
                    })
                    .with_effect(sdl2_particles::ParticleEffect::LinearMovement {
                        velocity_x: rng.gen_range(-500.0..500.0),
                        velocity_y: rng.gen_range(-500.0..500.0),
                    })
                    .build();
                    particles_state.emit(
                        1,
                        &particle_type,
                        transform.position.x + 40.0,
                        transform.position.y + 40.0,
                    );
                }
                *points += 1;
            }
        }
    }
    for crystal in crystals_to_delete.iter() {
        let _ = world.despawn(*crystal);
    }
}

fn system_ghost_ai(world: &mut hecs::World, player_death: &mut bool, dt: f32) {
    let mut optional_player_position = None;
    let mut optional_player_size = None;
    let mut should_die = false;
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
                should_die = true;
                break;
            }
        }
    }
    if should_die {
        player_damage(world, player_death);
    }
}

fn system_orbit_ai(state: &mut Level1State, dt: f32) {
    let mut optional_player_position = None;
    let mut optional_player_size = None;
    for (_id, (transform, sprite, _)) in &mut state.world.query::<(
        &components::Transform,
        &components::Sprite,
        &components::Player,
    )>() {
        optional_player_position = Some(transform.position);
        optional_player_size = Some(sprite.size);
    }
    if let (Some(target_pos), Some(_target_size)) = (optional_player_position, optional_player_size)
    {
        // ghost move
        for (_id, (transform, orbit_ai)) in state
            .world
            .query_mut::<(&mut components::Transform, &mut components::OrbitAI)>()
        {
            let difference = target_pos - transform.position;
            let x = transform.position.x;
            let y = transform.position.y;
            let dx = target_pos.x; // player x
            let dy = target_pos.y; // player y
            let r = difference.length();
            if r >= 250.0 && r <= 350.0 {
                orbit_ai.is_orbiting = true;
            } else {
                if r < 250.0 {
                    let delta_x = x - dx;
                    let delta_y = y - dy;
                    orbit_ai.angle = delta_y.atan2(delta_x).to_degrees();
                } else {
                    orbit_ai.is_orbiting = false;
                }
            }
            if orbit_ai.is_orbiting {
                let angle = (orbit_ai.angle).to_radians();
                orbit_ai.target_pos.x = dx + angle.cos() * 300.0;
                orbit_ai.target_pos.y = dy + angle.sin() * 300.0;
            }
            if r <= orbit_ai.radius_ghosting {
                if r >= 400.0 {
                    orbit_ai.target_pos = target_pos;
                }
                orbit_ai.velocity =
                    (orbit_ai.target_pos - transform.position).normalize() * orbit_ai.speed;
                transform.position += dt * orbit_ai.velocity;
            }
            if transform.position.distance(orbit_ai.target_pos) <= 10.0 {
                orbit_ai.angle += orbit_ai.angular_speed * dt;
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

fn system_bullets(world: &mut hecs::World, player_death: &mut bool, mob_count: &mut u32, dt: f32) {
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
    // Update bullet position
    for (_id, (transform, bullet)) in
        world.query_mut::<(&mut components::Transform, &components::Bullet)>()
    {
        transform.position += bullet.velocity * dt;
    }
    let mut bullets_ids_to_kill = vec![];
    let mut enemies_ids_to_kill = vec![];
    let mut should_die = false;
    for (bullet_id, (transform, sprite, bullet)) in &mut world.query::<(
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
            &components::Enemy,
        )>() {
            let enemy_rect = sdl2::rect::Rect::new(
                enemy_transform.position.x as i32,
                enemy_transform.position.y as i32,
                enemy_sprite.size.x,
                enemy_sprite.size.y,
            );
            if bullet.bullet_type == BulletType::FromPlayer
                && bullet_rect.has_intersection(enemy_rect)
            {
                bullets_ids_to_kill.push(bullet_id);
                enemies_ids_to_kill.push(enemy_id);
                break;
            }
            if let (Some(pos), Some(size)) = (optional_player_position, optional_player_size) {
                if bullet.bullet_type == BulletType::FromEnemy {
                    let player_rect =
                        sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, size.x, size.y);
                    if bullet_rect.has_intersection(player_rect) {
                        should_die = true;
                        bullets_ids_to_kill.push(bullet_id);
                    }
                }
            }
        }
    }
    if should_die {
        player_damage(world, player_death);
    }
    for bullet in bullets_ids_to_kill.iter() {
        let _ = world.despawn(*bullet);
    }

    for enemy in enemies_ids_to_kill.iter() {
        let _ = world.despawn(*enemy);
        *mob_count -= 1;
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
    particles_state: &mut sdl2_particles::ParticlesState,
    sound_shoot: &sdl2::mixer::Chunk,
    camera: &Camera,
    input_state: &InputState,
    rng: &mut ThreadRng,
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
        if controller.velocity.length() < 5.0 {
            controller.velocity = Vec2::ZERO;
        }
        controller.dashing_time_left -= dt;
        controller.dashing_timer -= dt;
        controller.attack_timer -= dt;
        player.invincibility_timer -= dt;
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
                let particle_type = sdl2_particles::ParticleTypeBuilder::new(
                    rng.gen_range(8..16),
                    rng.gen_range(8..16),
                    std::time::Duration::from_millis(rng.gen_range(100..200)),
                )
                .with_color(sdl2::pixels::Color::RGB(
                    rng.gen_range(111..131),
                    rng.gen_range(29..49),
                    rng.gen_range(25..45),
                )) // 121 39 35
                .with_effect(sdl2_particles::ParticleEffect::LinearRotation {
                    angular_velocity: 30.0,
                })
                .with_effect(sdl2_particles::ParticleEffect::FadeOut {
                    delay: std::time::Duration::ZERO,
                })
                .with_effect(sdl2_particles::ParticleEffect::LinearMovement {
                    velocity_x: -controller.velocity.x / 2.0 + rng.gen_range(-250.0..250.0),
                    velocity_y: -controller.velocity.y / 2.0 + rng.gen_range(-250.0..250.0),
                })
                .build();
                particles_state.emit(
                    1,
                    &particle_type,
                    transform.position.x + 40.0,
                    transform.position.y + 40.0,
                );
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
        create_bullet(world, pos, dir, BulletType::FromPlayer);
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

fn create_enemy_on(world: &mut hecs::World, x: i32, y: i32, rng: &mut ThreadRng) {
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
    if rng.gen_bool(0.0) {
        world.spawn((
            components::Transform::with_position(x as f32, y as f32),
            components::Sprite {
                filename: "res/snake.png",
                size: UVec2::new(40, 40),
            },
            components::GhostAI::default(),
            components::Enemy,
            enemy_animation_state,
        ));
    } else {
        world.spawn((
            components::Transform::with_position(x as f32, y as f32),
            components::Sprite {
                filename: "res/snake.png",
                size: UVec2::new(40, 40),
            },
            components::OrbitAI::default(),
            components::ShootingEnemy::default(),
            components::Enemy,
            enemy_animation_state,
        ));
    }
}

fn create_bullet(
    world: &mut hecs::World,
    position: Vec2,
    direction: Vec2,
    bullet_type: BulletType,
) {
    let speed = 64.0 * 15.0;
    world.spawn((
        components::Transform::with_position(position.x, position.y),
        components::Sprite {
            filename: "res/bullet.png",
            size: UVec2::new(16, 16),
        },
        components::Bullet {
            velocity: direction * speed,
            bullet_type,
        },
    ));
}
