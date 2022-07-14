use bevy::input::mouse::{MouseButtonInput};
use bevy::prelude::*;
use maze_walker::Point;

use crate::{MapMaze, MAP_SCALE};

// ANCHOR: mouse-button-input
fn _mouse_button_input(buttons: Res<Input<MouseButton>>) {
    if buttons.just_pressed(MouseButton::Left) {
        // Left button was pressed
    }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
    }
    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down
    }
    // we can check multiple at once with `.any_*`
    if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        // Either the left or the right button was just pressed
    }
}
// ANCHOR_END: mouse-button-input

// ANCHOR: mouse-button-events
fn _mouse_button_events(mut mousebtn_evr: EventReader<MouseButtonInput>) {
    use bevy::input::ElementState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ElementState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}
// ANCHOR_END: mouse-button-events

pub fn cursor_position(
    maze: Res<MapMaze>,
    windows: Res<Windows>,
    window_description: Res<WindowDescriptor>,
    buttons: Res<Input<MouseButton>>,
) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    let (_width, height) = (
        window_description.width as usize / MAP_SCALE as usize,
        window_description.height as usize / MAP_SCALE as usize,
    );

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            let point = Point {
                x: position[0] as usize / MAP_SCALE as usize,
                y: height - (position[1] as usize / MAP_SCALE as usize + 1),
            };
            dbg!(point);

            if let Some(connections) = maze.0.get_point_connections(&point) {
                dbg!(connections);
            }
        } else {
            // cursor is not inside the window
        }
    }
}