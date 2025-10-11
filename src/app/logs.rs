use bevy::{log::once, prelude::*};

pub fn entro() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy::log::LogPlugin {
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, log_system)
        .add_systems(Update, log_once_system)
        .add_systems(Update, panic_on_p)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("Press P to panic"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        }
    ));
}

fn log_system() {
    // trace!("very noisy");
    // debug!("helpful for debugging");
    // info!("helpful information that is worth printing by default");
    // warn!("something bad happened that isn't a failure, but thats worth calling out");
    // error!("something failed");
}

fn log_once_system() {
    trace_once!("one time noisy message");
    debug_once!("one time debug message");
    info_once!("some info which is printed only once");
    warn_once!("some warning we wish to call out only once");
    error_once!("some error we wish to report only once");
    for i in 0..10 {
        info_once!("logs once per call site, so this works just fine: {}", i);
    }
    once!({
        info!("doing expensive things");
        let mut a: u64 = 0;
        for i in 0..100000000 {
            a += i;
        }
        info!("result of some expensive one time calculation: {}", a);
    });
}

fn panic_on_p(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyP) {
        panic!("P pressed, panicking");
    }
}
