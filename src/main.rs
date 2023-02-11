use crate::{input::InputState, level1::Level1State};

mod components;
mod input;
mod player_state;
mod render;
mod texturemanager;

mod level1;

pub enum Level {
    Intro,
    _Menu,
    Level1,
}

fn main() {
    const VERSION: u32 = 1;
    #[cfg(feature = "puffin")]
    let _puffin_server = puffin_http::Server::new(&"0.0.0.0:8585").unwrap();
    #[cfg(feature = "puffin")]
    puffin::set_scopes_on(true);

    println!("Welcome in console linux user/developer :)");

    let sdl_context = sdl2::init().unwrap();
    let _image_context = sdl2::image::init(sdl2::image::InitFlag::all()).unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _audio_subsystem = sdl_context.audio().unwrap();
    sdl2::mixer::open_audio(
        sdl2::mixer::DEFAULT_FREQUENCY,
        sdl2::mixer::DEFAULT_FORMAT,
        sdl2::mixer::DEFAULT_CHANNELS,
        1024,
    )
    .unwrap(); // 1024 is default
    let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::all()).unwrap();
    sdl2::mixer::allocate_channels(16); // how many sounds can play simultaneously
    let mut sound_win = sdl2::mixer::Chunk::from_file("res/win.wav").unwrap();
    sound_win.set_volume(10);

    let window = video_subsystem
        .window("Psytumn", 1280, 720)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut input_state = InputState::new();
    let mut dt;
    let mut dt_timer = time::Instant::now();
    let mut fps_timer = time::Instant::now();
    let mut fps_counter = 0;
    let mut fps = 0;

    let mut level = Level::Level1;
    let mut level1_state = Level1State::new(&mut canvas);

    loop {
        puffin::GlobalProfiler::lock().new_frame();
        puffin::profile_scope!("main_loop");
        input_state.handle_events(&mut event_pump);
        if input_state.quit {
            break;
        }

        let now = time::Instant::now();
        dt = (now - dt_timer).as_seconds_f32();
        dt_timer = now;
        if now - fps_timer >= time::Duration::SECOND {
            fps_timer = now;
            fps = fps_counter;
            fps_counter = 0;
        }
        fps_counter += 1;
        let _ = canvas.window_mut().set_title(&format!(
            "Psytumn    FPS: {}    Version: {}    Date: {}",
            fps,
            VERSION,
            time::OffsetDateTime::now_utc().date()
        ));

        match level {
            Level::Intro => {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(3, 0, 52));
                canvas.clear();
                level1_state = Level1State::new(&mut canvas);
                let _ = sdl2::mixer::Channel::all().play(&sound_win, 0);
                level = Level::Level1;
                canvas.present();
            }
            Level::_Menu => todo!(),
            Level::Level1 => {
                level1::update(&mut level1_state, dt, &input_state, &mut level);
                level1::render(&mut level1_state, &mut canvas);
            }
        }
    }
}
