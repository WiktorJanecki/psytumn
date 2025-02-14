use glam::{UVec2, Vec2};
use sdl2_animation::AnimationState;

use crate::player_state;

#[derive(Clone)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2 { x: 1.0, y: 1.0 },
        }
    }
}

impl Transform {
    pub fn with_position(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        }
    }
}

pub struct Animation {
    pub state: AnimationState,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            state: AnimationState::new(),
        }
    }
}

pub struct Sprite {
    pub filename: &'static str,
    pub size: UVec2,
}

#[derive(PartialEq)]
pub enum BulletType {
    FromPlayer,
    FromEnemy,
}
pub struct Bullet {
    pub velocity: Vec2,
    pub bullet_type: BulletType,
}

pub struct Player {
    pub state_machine: player_state::StateMachine,
    pub lives: u8,
    pub invincibility_timer: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state_machine: player_state::StateMachine {
                state: player_state::State::Idle,
                dashing_time_left: 0.0,
                dashing_cooldown_timer: 0.0,
            },
            lives: 3,
            invincibility_timer: 0.0,
        }
    }
}

pub struct CameraTarget;

pub struct Enemy;

pub struct GhostAI {
    pub velocity: Vec2,
    pub speed: f32,
    pub radius: f32,
}

pub struct OrbitAI {
    pub velocity: Vec2,
    pub speed: f32,
    pub angular_speed: f32,
    pub radius_ghosting: f32,
    pub radius_orbiting: f32,
    pub angle: f32,
    pub is_orbiting: bool,
    pub target_pos: Vec2,
}

pub struct ShootingEnemy {
    pub timer: f32,
    pub cooldown: f32,
    pub range: f32,
}

impl Default for ShootingEnemy {
    fn default() -> Self {
        Self {
            timer: 1.0,
            cooldown: 1.0,
            range: 400.0,
        }
    }
}

impl Default for OrbitAI {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            speed: 400.0,
            angular_speed: 90.0, // deg per sec
            radius_ghosting: 512.0 + 128.0,
            radius_orbiting: 256.0,
            angle: 0.0,
            is_orbiting: false,
            target_pos: Vec2::ZERO,
        }
    }
}

impl Default for GhostAI {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            speed: 256.0,
            radius: 512.0 + 128.0,
        }
    }
}
pub struct PlayerController {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dashing_timer: f32,
    pub dashing_time_left: f32,
    pub attack_timer: f32,
    pub can_move: bool,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            dashing_timer: 0.0,
            dashing_time_left: 0.0,
            attack_timer: 0.0,
            can_move: true,
        }
    }
}

pub struct DashingCrystal;
pub struct PointCrystal;
