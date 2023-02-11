pub struct StateMachine {
    pub state: State,
    pub dashing_time_left: f32,
    pub dashing_cooldown_timer: f32,
}
pub enum Input {
    Move,
    Dash,
    Nothing,
    Crystal,
}

pub enum State {
    Idle,
    Moving,
    Dashing,
    Stopped,
}

pub fn handle_state(
    state_machine: &mut StateMachine,
    input: &Input,
    sound_dash: &sdl2::mixer::Chunk,
    dt: f32,
) {
    let dashing_cooldown = 0.5;
    let dashing_time = 0.2;
    state_machine.dashing_cooldown_timer -= dt;
    state_machine.dashing_time_left -= dt;
    match state_machine.state {
        State::Idle => match input {
            Input::Move => state_machine.state = State::Moving,
            _ => {}
        },
        State::Moving => match input {
            Input::Dash => {
                if state_machine.dashing_cooldown_timer <= 0.0 {
                    state_machine.state = State::Dashing;
                    let _ = sdl2::mixer::Channel::all().play(sound_dash, 0);
                    state_machine.dashing_cooldown_timer = dashing_cooldown;
                    state_machine.dashing_time_left = dashing_time;
                }
            }
            Input::Nothing => {
                state_machine.state = State::Idle;
            }
            Input::Crystal => {
                state_machine.state = State::Stopped;
                state_machine.dashing_cooldown_timer = 0.0;
            }
            _ => {}
        },
        State::Dashing => match input {
            Input::Crystal => {
                state_machine.state = State::Stopped;
                state_machine.dashing_cooldown_timer = 0.0;
            }
            _ => {
                if state_machine.dashing_time_left <= 0.0 {
                    state_machine.state = State::Idle;
                }
            }
        },
        State::Stopped => match input {
            Input::Dash => {
                if state_machine.dashing_cooldown_timer <= 0.0 {
                    state_machine.state = State::Dashing;
                    let _ = sdl2::mixer::Channel::all().play(sound_dash, 0);
                    state_machine.dashing_cooldown_timer = dashing_cooldown;
                    state_machine.dashing_time_left = dashing_time;
                }
            }
            _ => {}
        },
    }
}
