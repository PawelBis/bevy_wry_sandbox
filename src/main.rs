use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::PURPLE))
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_wry::BevyWryPlugin {
            as_child: true,
            url: Some("file:///Users/aoamne/Source/bevy_wry_sandbox/web/index.html".to_owned()),
            html: None,
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
