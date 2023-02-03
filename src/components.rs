use glam::{Vec2, UVec2};
use sdl2_animation::AnimationState;

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