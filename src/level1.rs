pub struct Level1State{
    have_started: bool,
}

impl Level1State{
    pub fn new() -> Self{
        Self{
            have_started: false,
        }
    }
}

pub fn update(state: &mut Level1State, _dt: f32){
    if !state.have_started{
        state.have_started = true;
        println!("void Start(){{}}");
    }
}

pub fn render(_state: &Level1State, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RED);
    canvas.clear();
    canvas.present();
}