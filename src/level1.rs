pub fn update(_dt: f32){

}

pub fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(sdl2::pixels::Color::RED);
    canvas.clear();
    canvas.present();
}