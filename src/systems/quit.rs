use bevy::prelude::*;
use bevy::app::AppExit;

pub fn quit_on_escape(
    key_in: Res<ButtonInput<KeyCode>>,
    mut exit_writer: EventWriter<AppExit>
) {
    if key_in.just_pressed(KeyCode::Escape) {
        exit_writer.write(AppExit::Success);
    }
}

