use bevy::{prelude::*, render::camera::Viewport, utils};
use bevy_asm::BevyAsmPlugin;
use bevy_wry::UrlResource;
use viewport::{update_viewport, EditorViewportCamera, EditorViewportUpdated};

mod viewport;

fn main() {
    App::new()
        .init_resource::<EditorViewportUpdated>()
        .insert_resource(ClearColor(Color::PURPLE))
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_wry::BevyWryPlugin {
            url: UrlResource("http://localhost:5173".to_owned()),
            ..default()
        })
        .add_plugins(BevyAsmPlugin {
            use_in_memory_db: true,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_viewport.map(utils::error))
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
