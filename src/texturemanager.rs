use std::collections::HashMap;

use sdl2::{
    image::LoadTexture,
    render::{Texture, TextureCreator},
    video::WindowContext,
};

pub struct TextureManager {
    textures: HashMap<&'static str, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }
    pub fn texture(
        self: &mut Self,
        filename: &'static str,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> &Texture {
        if !self.textures.contains_key(filename) {
            let texture = texture_creator.load_texture(filename).unwrap(); // if texture does not exist imo panic (maybe TODO: create missing texture texture)
            self.textures.insert(filename, texture);
        }
        return self.textures.get(filename).unwrap();
    }
    pub fn texture_mut(
        self: &mut Self,
        filename: &'static str,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> &mut Texture {
        if !self.textures.contains_key(filename) {
            let texture = texture_creator.load_texture(filename).unwrap(); // if texture does not exist imo panic (maybe TODO: create missing texture texture)
            self.textures.insert(filename, texture);
        }
        return self.textures.get_mut(filename).unwrap();
    }
}
