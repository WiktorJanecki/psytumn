use sdl2::{render::{TextureCreator}, video::WindowContext};
use sdl2_animation::{AnimationState, Keyframe, Animation};

use crate::{texturemanager::{TextureManager}, input::InputState};

pub struct Level1State{
    update_started: bool,
    texture_creator: TextureCreator<WindowContext>,
    texture_manager: TextureManager,
    player_animation_state: AnimationState,
}

impl Level1State{
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Self{
        Self{
            update_started: false,
            texture_creator: canvas.texture_creator(),
            texture_manager: TextureManager::new(),
            player_animation_state: AnimationState::new(),
        }
    }
}

pub fn update(state: &mut Level1State, dt: f32, input_state: &InputState){
    if !state.update_started{
        let idle_animation: Animation = vec![
            Keyframe{ x: 0, y: 0, width: 40, height: 40, duration: std::time::Duration::from_secs(1) },
            Keyframe{ x: 40, y: 0, width: 40, height: 40, duration: std::time::Duration::from_secs(1) }
        ];
        state.player_animation_state.play(&idle_animation);
        state.update_started = true;
        println!("void Start(){{}}");
    }
    state.player_animation_state.update(std::time::Duration::from_secs_f32(dt));
}

pub fn render(state: &mut Level1State, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {    
    canvas.set_draw_color(sdl2::pixels::Color::RGB(39,9,31));
    canvas.clear();
    let scale = 3;
    let texture = state.texture_manager.texture("player.png", &state.texture_creator);
    let _ = canvas.copy(texture, state.player_animation_state.get_src(), sdl2::rect::Rect::new(64,64,40*scale,40*scale));
    canvas.present();
}