use sdl2::{render::TextureCreator, video::WindowContext};

use crate::{input::InputState, texturemanager::TextureManager, Level};

pub struct IntroState {
    update_started: bool,
    texture_creator: TextureCreator<WindowContext>,
    texture_manager: TextureManager,
    slides: Vec<IntroSlide>,
    timer: f32,
}

struct IntroSlide{
    opacity: f32, // 0->1
    texture: &'static str,
}

impl IntroState {
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Self {
        Self {
            update_started: false,
            texture_creator: canvas.texture_creator(),
            texture_manager: TextureManager::new(),
            timer: 0.0,
            slides: vec![],
        }
    }
}

pub fn update(state: &mut IntroState, dt: f32, _input_state: &mut InputState, level: &mut Level) {
    if !state.update_started {
        state.update_started = true;
        state.slides = vec![IntroSlide{opacity: 0.0, texture: &"res/intro1.png"}, //
                            IntroSlide{opacity: 0.0, texture: &"res/intro2.png"}, //
                            IntroSlide{opacity: 0.0, texture: &"res/intro3.png"}];//
    }
    state.timer += dt;

    for (index, slide) in state.slides.iter_mut().enumerate(){
        slide.opacity = get_opacity(state.timer, index);
    }

    let slide_count = state.slides.len();
    let after_slideshow_delay = 1.0;
    if state.timer >= slide_count as f32 * SLIDE_TIME + after_slideshow_delay{
        *level = Level::Menu;
    }
}
const SLIDE_TIME:f32 = 4.0;
fn get_opacity(t: f32, index: usize) -> f32 {
    let slide_time = SLIDE_TIME;
    let slide_in_time = 0.2;
    let slide_out_time = 1.0;
    let delay = slide_time;

    let offset = delay * index as f32; // slide starting point
    if t >= offset && t <= offset + slide_time{
        if t <= offset + slide_in_time{ // ease in 
            let ease_percent = (t - offset) / slide_in_time;
            return easey::f32::ease_out_circ(ease_percent);
        }
        else if t  <= offset + slide_time-slide_out_time{ // opacity 1
            1.0
        }
        else{ // ease out
            let ease_percent = (t - offset - (slide_time-slide_out_time)) / slide_out_time;
            return 1.0 - easey::f32::ease_in_out(ease_percent);
        }
    }
    else{
        0.0
    }
}

pub fn render(state: &mut IntroState, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::BLACK);
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.clear();
    for slide in state.slides.iter(){
        let dst = sdl2::rect::Rect::new(0,0,1280,720);
        let texture = state
            .texture_manager
            .texture_mut(slide.texture, &state.texture_creator);
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        texture.set_alpha_mod((slide.opacity * 255.0) as u8);
        let _ = canvas.copy(texture, None, dst);
    }

    canvas.present();
}