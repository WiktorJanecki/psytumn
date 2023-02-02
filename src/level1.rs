use sdl2::{render::{TextureCreator}, video::WindowContext};

use crate::texturemanager::{TextureManager};

pub struct Level1State{
    update_started: bool,
    texture_creator: TextureCreator<WindowContext>,
    texture_manager: TextureManager,
}

impl Level1State{
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Self{
        Self{
            update_started: false,
            texture_creator: canvas.texture_creator(),
            texture_manager: TextureManager::new(),
        }
    }
}

pub fn update(state: &mut Level1State, _dt: f32){
    if !state.update_started{

        // data loading


        state.update_started = true;
        println!("void Start(){{}}");
    }
}

pub fn render(state: &mut Level1State, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {    
    canvas.set_draw_color(sdl2::pixels::Color::RGB(39,9,31));
    canvas.clear();
    let scale = 3;
    let texture = state.texture_manager.texture("player.png", &state.texture_creator);
    let _ = canvas.copy(texture, None, sdl2::rect::Rect::new(64,64,64*scale,32*scale));
    canvas.present();
}