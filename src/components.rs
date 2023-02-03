use glam::{Vec2, UVec2};
use sdl2_animation::AnimationState;

#[derive(Clone)]
pub struct Transform{
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform{
    fn default() -> Self {
        Self { position: Vec2::ZERO, rotation: 0.0, scale: Vec2{x: 1.0, y: 1.0}}
    }
}

impl Transform{
    pub fn with_position(x: f32, y: f32) -> Self{
        Self { position: Vec2::new(x,y), rotation: 0.0, scale: Vec2::new(1.0,1.0) }
    }
}


pub struct Animation{
    pub state: AnimationState,
}

impl Default for Animation{
    fn default() -> Self {
        Self { state: AnimationState::new() }
    }
}

pub struct Sprite{
    pub filename: &'static str,
    pub size: UVec2,
}

pub struct Player;
pub struct CameraTarget;

pub struct GhostAI{
    pub velocity: Vec2,
    pub speed: f32,
    pub radius: f32,
}

impl Default for GhostAI{
    fn default() -> Self {
        Self { velocity: Vec2::ZERO, speed: 256.0, radius: 512.0 + 128.0 }
    }
}
pub struct PlayerController{
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dashing_timer: f32,
    pub dashing_time_left: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self { velocity: Vec2::ZERO, acceleration: Vec2::ZERO, dashing_timer: 0.0, dashing_time_left: 0.0  }
    }
}