use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseButtonInput};

// ANCHOR: mouse-button-input
fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
) {
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
fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
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

// ANCHOR: mouse-motion
fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
) {
    for ev in motion_evr.iter() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}
// ANCHOR_END: mouse-motion