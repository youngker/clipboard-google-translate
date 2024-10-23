mod clipboard;
use bevy::prelude::*;
#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;
use bevy::winit::{EventLoopProxy, WinitSettings};
use bevy::winit::WakeUp;
use bevy::color::palettes::css::GREEN_YELLOW;
use crossbeam_channel::{bounded, Receiver};
use std::thread;
use std::time::Duration;

pub fn start() {
    App::new()
        .add_event::<StreamEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: (800., 300.).into(),
//                transparent: true,
//                decorations: false,
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .add_systems(Update, (read_stream, spawn_text))
        .run();
}

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<String>);

#[derive(Event)]
struct StreamEvent(String);

#[derive(Component)]
struct Popup(String);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/NotoSansCJK-Regular.ttc"),
        font_size: 24.0,
        ..default()
    };
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(TextBundle {
            text: Text::from_section("init", text_style.clone()).with_justify(JustifyText::Center),
            style: Style {
                margin: UiRect::bottom(Val::Px(10.)),
                ..Default::default()
            },
            background_color: GREEN_YELLOW.into(),
            ..Default::default()
        })
        .insert(Popup("init".to_string()));

    let (tx, rx) = bounded::<String>(10);
    thread::spawn(move || {
        let mut clipboard = clipboard::ClipboardThread::new();
        let receiver = clipboard.start();
        loop {
            if let Ok(received) = receiver.try_recv() {
                if clipboard.text != received {
                    if received.trim() == "" {
                        println!("this is a null");
                        clipboard.set_text(received.clone());
                    } else {
                        println!("got: {}", received);
                        clipboard.set_text(received.clone());
                        tx.send(clipboard.request()).unwrap();
                    }
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    });
    commands.insert_resource(StreamReceiver(rx));
}

fn read_stream(receiver: Res<StreamReceiver>, mut events: EventWriter<StreamEvent>) {
    for from_stream in receiver.try_iter() {
        events.send(StreamEvent(from_stream));
        println!("read_stream=>send event!");
    }
}

fn spawn_text(
    mut windows: Query<&mut Window>,
    mut query: Query<(&mut Text, &Popup)>,
    mut reader: EventReader<StreamEvent>,
    event_loop_proxy: NonSend<EventLoopProxy<WakeUp>>,
) {
    for (mut text, name) in query.iter_mut() {
        for (_per_frame, event) in reader.read().enumerate() {
            println!("spawn_text");
            text.sections[0].value = format!("{}:{}", name.0, event.0);
            let mut window = windows.single_mut();
            window.resolution.set(500.0, 100.0);
            let _ = event_loop_proxy.send_event(WakeUp);
            WinitSettings::desktop_app();
        }
    }
}
