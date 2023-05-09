use glam::Vec2;

use crate::{components, render::Camera, SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn system_camera_follow(world: &hecs::World, camera: &mut Camera, dt: f32) {
    for (_id, (transform, _target, sprite)) in
        &mut world.query::<(&components::Transform, &components::CameraTarget, &components::Sprite)>()
    {
        let target_width = sprite.size.x as f32;
        let target_height = sprite.size.y as f32;
        let smooth_value = 15.0;
        let offset = Vec2::new(-(SCREEN_WIDTH as f32) / 2.0 + target_width / 2.0, -(SCREEN_HEIGHT as f32) / 2.0 + target_height / 2.0);
        let target_position = transform.position + offset;
        let smoothed_position = camera.position.lerp(target_position, smooth_value * dt);
        camera.position = smoothed_position;
    }
}

