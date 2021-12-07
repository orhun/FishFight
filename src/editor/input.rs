use macroquad::{experimental::collections::storage, prelude::*};

use fishsticks::{Axis, Button};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorInputScheme {
    Mouse,
    Gamepad(fishsticks::GamepadId),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct EditorInput {
    pub action: bool,
    pub back: bool,
    pub context_menu: bool,
    pub camera_move_direction: Vec2,
    pub camera_mouse_move: bool,
    pub camera_zoom: f32,
    pub cursor_move_direction: Vec2,
    pub undo: bool,
    pub redo: bool,
    pub toggle_menu: bool,
    pub toggle_draw_grid: bool,
    pub toggle_snap_to_grid: bool,
    pub toggle_disable_parallax: bool,
    pub save: bool,
    pub save_as: bool,
    pub load: bool,
}

pub fn collect_editor_input(scheme: EditorInputScheme) -> EditorInput {
    let mut input = EditorInput::default();

    match scheme {
        EditorInputScheme::Mouse => {
            input.action = is_mouse_button_down(MouseButton::Left);
            input.camera_mouse_move = is_mouse_button_down(MouseButton::Middle);
            input.context_menu = is_mouse_button_pressed(MouseButton::Right);

            let (_, zoom) = mouse_wheel();
            if zoom < 0.0 {
                input.camera_zoom = -1.0;
            } else if zoom > 0.0 {
                input.camera_zoom = 1.0;
            }

            if is_key_down(KeyCode::LeftControl) {
                if is_key_pressed(KeyCode::Z) {
                    if is_key_down(KeyCode::LeftShift) {
                        input.redo = true;
                    } else {
                        input.undo = true;
                    }
                }

                input.toggle_snap_to_grid = is_key_pressed(KeyCode::G);

                if is_key_pressed(KeyCode::S) {
                    if is_key_down(KeyCode::LeftShift) {
                        input.save_as = true;
                    } else {
                        input.save = true;
                    }
                }

                if is_key_pressed(KeyCode::L) {
                    input.load = true;
                }
            } else {
                if is_key_pressed(KeyCode::Escape) {
                    input.toggle_menu = true;
                    input.back = true;
                }

                if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    input.camera_move_direction.x = -1.0;
                } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    input.camera_move_direction.x = 1.0;
                }

                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                    input.camera_move_direction.y = -1.0;
                } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                    input.camera_move_direction.y = 1.0;
                }

                input.toggle_draw_grid = is_key_pressed(KeyCode::G);

                input.toggle_disable_parallax = is_key_pressed(KeyCode::P);
            }
        }
        EditorInputScheme::Gamepad(ix) => {
            let gamepad_system = storage::get_mut::<fishsticks::GamepadContext>();
            let gamepad = gamepad_system.gamepad(ix);

            if let Some(gamepad) = gamepad {
                input.action = gamepad.digital_inputs.activated(Button::B);
                input.back = gamepad.digital_inputs.activated(Button::A);
                input.context_menu = gamepad.digital_inputs.activated(Button::X);

                input.camera_move_direction = {
                    let direction_x = gamepad.analog_inputs.value(Axis::LeftX);
                    let direction_y = gamepad.analog_inputs.value(Axis::LeftY);

                    let direction = vec2(direction_x, direction_y);

                    direction.normalize_or_zero()
                };

                input.cursor_move_direction = {
                    let direction_x = gamepad.analog_inputs.value(Axis::RightX);
                    let direction_y = gamepad.analog_inputs.value(Axis::RightY);

                    let direction = vec2(direction_x, direction_y);

                    direction.normalize_or_zero()
                };
            }
        }
    }

    input
}
