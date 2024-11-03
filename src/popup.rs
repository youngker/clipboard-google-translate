mod clipboard;

#[cfg(target_os = "macos")]
use bevy::window::CompositeAlphaMode;

use bevy::{
    color::palettes::css::*,
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
    window::Cursor,
    winit::{EventLoopProxy, WakeUp, WinitSettings},
};
use bevy_prototype_lyon::prelude::*;
use crossbeam_channel::{bounded, Receiver};
use std::thread;
use std::time::Duration;

pub fn start() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_event::<StreamEvent>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                resolution: (900., 300.).into(),
                transparent: true,
                decorations: false,
                cursor: Cursor {
                    hit_test: false,
                    ..default()
                },
                #[cfg(target_os = "macos")]
                composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .add_systems(Update, (read_stream, spawn_text, handle_click))
        .run();
}

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<String>);

#[derive(Event)]
struct StreamEvent(String);

#[derive(Component)]
struct TranslatedText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // Demonstrate text wrapping
    let font = asset_server.load("fonts/NotoSansCJK-Regular.ttc");
    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 24.0,
        ..default()
    };

    let width = 900.0;
    let height = 150.0;
    let tail_width = 20.0;
    let tail_height = 20.0;

    let points = [
        Vec2::new(-width / 2.0, height / 2.0),
        Vec2::new(width / 2.0, height / 2.0),
        Vec2::new(width / 2.0, -height / 2.0),
        Vec2::new(-tail_width / 2.0, -height / 2.0),
        Vec2::new(0.0, -height / 2.0 - tail_height),
        Vec2::new(tail_width / 2.0, -height / 2.0),
        Vec2::new(-width / 2.0, -height / 2.0),
    ];

    let shape = shapes::Polygon {
        points: points.into_iter().collect(),
        //        radius: 10.,
        closed: false,
    };
    let box_size = Vec2::new(800.0, 130.0);

    //    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(DARK_CYAN),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "this text wraps in the box\n(Unicode linebreaks)",
                            slightly_smaller_text_style.clone(),
                        )],
                        justify: JustifyText::Left,
                        linebreak_behavior: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds {
                        // Wrap text in the rectangle
                        size: box_size,
                    },
                    // ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                },
                TranslatedText,
            ));
        });

    // commands
    //     .spawn(TextBundle {
    //         text: Text::from_section("init", text_style.clone()).with_justify(JustifyText::Center),
    //         style: Style {
    //             margin: UiRect::bottom(Val::Px(10.)),
    //             ..Default::default()
    //         },
    //         background_color: Color::BLACK.into(),
    //         ..Default::default()
    //     })
    //     .insert(Popup("init".to_string()));

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
    mut query: Query<&mut Text, With<TranslatedText>>,
    mut reader: EventReader<StreamEvent>,
    event_loop_proxy: NonSend<EventLoopProxy<WakeUp>>,
) {
    for mut text in &mut query {
        for (_per_frame, event) in reader.read().enumerate() {
            println!("spawn_text");
            text.sections[0].value = format!("{}", event.0);
            // let mut window = windows.single_mut();
            // window.resolution.set(500.0, 100.0);
            // let _ = event_loop_proxy.send_event(WakeUp);
            // WinitSettings::desktop_app();
        }
    }
}

fn handle_click(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    if buttons.pressed(MouseButton::Left) {
        println!("pressed left button");
    }
}
