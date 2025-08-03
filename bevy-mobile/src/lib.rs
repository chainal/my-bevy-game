use bevy::{
    color::palettes::basic::*,
    input::{gestures::RotationGesture, touch::TouchPhase},
    log::{Level, LogPlugin},
    prelude::*,
    window::{AppLifecycle, WindowMode},
    winit::WinitSettings,
};


// the `bevy_main` proc_macro generates the required boilerplate for Android
#[bevy_main]
/// The entry point for the application. Is `pub` so that it can be used from
/// `main.rs`.
pub fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    recognize_rotation_gesture: true,
                    prefers_home_indicator_hidden: true,
                    prefers_status_bar_hidden: true,
                    ..default()
                }),
                ..default()
            }),
    )
    .insert_resource(WinitSettings::mobile())
    .add_systems(Startup, (setup_scene, setup_music))
    .add_systems(Update, (
        touch_camera,
        button_handler,
        handle_lifetime.run_if(any_with_component::<AudioSink>)
    ))
    .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.2, 0.1)))
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.4, 0.3))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap())),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.4, 0.8))),
        Transform::from_xyz(1.5, 1.5, 1.5),
    ));
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            #[cfg(not(target_os = "android"))]
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        #[cfg(target_os = "android")]
        Msaa::Off,
    ));
    commands
        .spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(50.0),
                right: Val::Px(50.0),
                bottom: Val::Px(50.0),
                ..default()
            }
        ))
        .with_child((
            Text::new("Test Button"),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor::BLACK,
            TextLayout::new_with_justify(JustifyText::Center),
        ));
}

fn setup_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
        PlaybackSettings::LOOP,
    ));
}

fn touch_camera(
    window: Query<&Window>,
    mut touches: EventReader<TouchInput>,
    mut camera_transform: Single<&mut Transform, With<Camera3d>>,
    mut last_position: Local<Option<Vec2>>,
    mut rotations: EventReader<RotationGesture>,
) {
    let Ok(window) = window.single() else {
        return;
    };
    for touch in touches.read() {
        if touch.phase == TouchPhase::Started {
            *last_position = None;
        }
         if let Some(last_position) = *last_position {
            **camera_transform = Transform::from_xyz(
                camera_transform.translation.x + (touch.position.x - last_position.x) / window.width() * 5.0,
                camera_transform.translation.y,
                camera_transform.translation.z + (touch.position.y - last_position.y) / window.height() * 5.0,
            )
            .looking_at(Vec3::ZERO, Vec3::Y);
         }
         *last_position = Some(touch.position);
    }
    for rotation in rotations.read() {
        let forward = camera_transform.forward();
        camera_transform.rotate_axis(forward, rotation.0 / 10.0);
    }
}

fn button_handler(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BLUE.into();
            },
            Interaction::Hovered => {
                *color = GRAY.into();
            },
            Interaction::None => {
                *color = WHITE.into();
            }
        }
    }
}

fn handle_lifetime(
    mut lifecycle_events: EventReader<AppLifecycle>,
    music_controller: Single<&AudioSink>,
) {
    for event in lifecycle_events.read() {
        match event {
            AppLifecycle::Idle | AppLifecycle::WillSuspend | AppLifecycle::WillResume => {},
            AppLifecycle::Suspended => music_controller.pause(),
            AppLifecycle::Running => music_controller.play(),
        }
    }
}