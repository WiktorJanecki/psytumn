use glam::Vec2;
use sdl2_animation::AnimationState;

pub struct Transform{
    position: Vec2,
    rotation: f32,
    scale: Vec2,
}

impl Default for Transform{
    fn default() -> Self {
        Self { position: Vec2::ZERO, rotation: 0.0, scale: Vec2{x: 1.0, y: 1.0}}
    }
}


pub struct Animation{
    state: AnimationState,
}

impl Default for Animation{
    fn default() -> Self {
        Self { state: AnimationState::new() }
    }
}