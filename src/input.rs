use glam::Vec2;
use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, EventPump};

pub struct InputState {
    pub movement: Vec2,
    pub shooting: Vec2, // TODO: not implemented
    pub attack: bool,
    pub dash: bool,
    pub quit: bool,

    l: bool, //left
    r: bool, //right
    u: bool, //up
    d: bool, //down helper variables
}

impl InputState {
    pub fn new() -> Self {
        Self {
            movement: Vec2::ZERO,
            shooting: Vec2::ZERO,
            attack: false,
            dash: false,
            quit: false,
            l: false,
            r: false,
            u: false,
            d: false,
        }
    }
    pub fn handle_events(self: &mut Self, pump: &mut EventPump) {
        self.dash = false;
        self.attack = false;
        self.movement = Vec2::ZERO;

        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.dash = true,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => self.u = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.d = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => self.l = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => self.r = true,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => self.u = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => self.d = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => self.l = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => self.r = false,
                Event::MouseButtonDown { mouse_btn: btn, .. } => {
                    if btn == MouseButton::Left {
                        self.attack = true
                    }
                }
                _ => {}
            }
        }
        let movement_x = if self.l && self.r {
            -1.0
        } else if self.l {
            -1.0
        } else if self.r {
            1.0
        } else {
            0.0
        };
        let movement_y = if self.u && self.d {
            -1.0
        } else if self.u {
            -1.0
        } else if self.d {
            1.0
        } else {
            0.0
        };
        self.movement = Vec2 {
            x: movement_x,
            y: movement_y,
        };
    }
}
