use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy_asm::BevyAsmPlugin;
use bevy_wry::components::Anchor;
use bevy_wry::events::{EmptyInEvent, WebViewEvent, CreateWebViewBuilder};

const FILE_EXPLORER_WEBVIEW: &str = "FileExplorer";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::PURPLE))
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_wry::BevyWryPlugin::<EmptyInEvent>::default())
        .add_plugins(BevyAsmPlugin {
            use_in_memory_db: true,
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut webview_event_writer: EventWriter<WebViewEvent>) {
    // Spawn big rectangle to debug camera viewport
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(1000.0, 1000.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });

    // Spawn bewry viewport camera
    commands.spawn((Camera2dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(200, 200),
                physical_size: UVec2::new(200, 200),
                ..default()
            }),
            ..default()
        },
        ..default()
    },));

    let create_webview = CreateWebViewBuilder::new(FILE_EXPLORER_WEBVIEW)
        .with_url("http://localhost:5174".to_string())
        .with_anchor(Anchor::BottomStretch)
        .build();
    webview_event_writer.send(WebViewEvent::Create(create_webview));
}
