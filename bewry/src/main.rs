use bevy::{prelude::*, render::camera::Viewport, utils};
use bevy_asm::BevyAsmPlugin;
use bevy_wry::communication::InEvent;
use types::EditorCommand;
use viewport::{update_viewport, EditorViewportCamera, EditorViewportUpdated};

mod types;
mod viewport;

fn main() {
    App::new()
        .init_resource::<EditorViewportUpdated>()
        .insert_resource(ClearColor(Color::PURPLE))
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_wry::SymmetricWryPlugin::<EditorCommand>::new(
            "http://localhost:5173".to_owned(),
        ))
        .add_plugins(BevyAsmPlugin {
            use_in_memory_db: true,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_viewport.map(utils::error))
        .add_systems(Update, consume_events)
        .run();
}

fn setup(mut commands: Commands) {
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
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                viewport: Some(Viewport {
                    physical_position: UVec2::new(200, 200),
                    physical_size: UVec2::new(200, 200),
                    ..default()
                }),
                ..default()
            },
            ..default()
        },
        EditorViewportCamera,
    ));
}

fn consume_events(
    mut reader: EventReader<InEvent<EditorCommand>>,
    mut viewport_updated: ResMut<EditorViewportUpdated>,
) {
    for event in reader.read() {
        match **event {
            EditorCommand::ResizeViewport {
                new_position,
                new_size,
            } => {
                *viewport_updated = EditorViewportUpdated {
                    new_position,
                    new_size,
                }
            }
        }
    }
}
