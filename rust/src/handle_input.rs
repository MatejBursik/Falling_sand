use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use crate::resources::MousePressed;

pub fn handle_input(
    mut mouse_events: EventReader<MouseButtonInput>,
    mut pressed: ResMut<MousePressed>
) {    
    for ev in mouse_events.read() {
        if ev.button == MouseButton::Left {
            match ev.state {
                ButtonState::Pressed => {
                    pressed.state = true;
                    
                }
                ButtonState::Released => {
                    pressed.state = false;
                }
            }
        }
    }
}
