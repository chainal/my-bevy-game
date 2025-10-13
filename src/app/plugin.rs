use bevy::prelude::*;
use core::time::Duration;

pub fn entro() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PrintMessagePlugin {
                wait_duration: Duration::from_secs(1),
                message: "This is an example plugin".to_string(),
            }
        ))
        .run();
}

struct PrintMessagePlugin {
    wait_duration: Duration,
    message: String,
}

impl Plugin for PrintMessagePlugin {
    fn build(&self, app: &mut App) {
        let state = PrintMessageState {
            message: self.message.clone(),
            timer: Timer::new(self.wait_duration, TimerMode::Repeating),
        };
        app.insert_resource(state)
            .add_systems(Update, print_message_system);
    }
}

#[derive(Resource)]
struct PrintMessageState {
    message: String,
    timer: Timer,
}

fn print_message_system(mut state: ResMut<PrintMessageState>, time: Res<Time>) {
    if state.timer.tick(time.delta()).just_finished() {
        info!("{}", state.message);
    }
}
