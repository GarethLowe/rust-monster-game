use ruscii::app::{App, State};
use ruscii::terminal::{Window};
use ruscii::drawing::{Pencil};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};
use ruscii::gui::{FPSCounter};

struct GameState {
    pub player_position: Vec2,
}

impl GameState {
    pub fn new(player_start: Vec2) -> Self {
        Self { player_position: player_start }
    }

    pub fn update(&mut self) {

    }
}

fn main() {
    let mut fps_counter = FPSCounter::new();
    let mut app = App::new();
    let mut state = GameState::new(Vec2::xy(0, 0));

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::W => state.player_position.y -= 1,
                Key::S => state.player_position.y += 1,
                Key::A => state.player_position.x -= 1,
                Key::D => state.player_position.x += 1,
                _ => (),
            }
        }

        fps_counter.update();
        if app_state.step() % 2 == 0 {
            state.update();
        }

        Pencil::new(window.canvas_mut())
            .draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(0, 0))
            .draw_text(&format!("POS: {}, {}", state.player_position.x, state.player_position.y), Vec2::xy(0, 1))
            .set_origin(Vec2::xy(state.player_position.x, state.player_position.y))
            .draw_text("☺", Vec2::zero());
    });
}