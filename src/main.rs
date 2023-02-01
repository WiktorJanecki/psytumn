
mod level1;

enum Level{
    Intro, 
    _Menu,
    Level1,
}

fn main() {
    const VERSION: u32 = 0;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Psytumn", 1280, 720)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut _dt;
    let mut dt_timer = time::Instant::now();
    let mut fps_timer = time::Instant::now();
    let mut fps_counter = 0;
    let mut fps = 0;

    let mut level = Level::Intro;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {break 'running;} 
                _ => {}
            }
        }
        
        let now = time::Instant::now();
        _dt = (now - dt_timer).as_seconds_f32();
        dt_timer = now;
        if now-fps_timer >= time::Duration::SECOND{
            fps_timer = now;
            fps = fps_counter;
            fps_counter = 0;
        }
        fps_counter+=1;
        let _ = canvas.window_mut().set_title(&format!("Psytumn    FPS: {}    Version: {}    Date: {}",fps, VERSION, time::OffsetDateTime::now_utc().date()));

        match level{
            Level::Intro => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(3, 0, 52));
                canvas.clear();
                if fps > 0 { // after a second
                    level = Level::Level1
                }
                canvas.present();
            },
            Level::_Menu => todo!(),
            Level::Level1 => {
                level1::update(_dt);
                level1::render(&mut canvas);
            },
        }
    }
    println!("Hello, world!");
}
