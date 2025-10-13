use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
};

pub fn entro() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: None,
                    ..default()
                }.into(),
                ..default()
            })
        )
        .run();
}