use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::PURPLE))
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_wry::BevyWryPlugin {
            as_child: true,
            url: None,
            html: Some(
                r#"<html>
                  <body style="background-color:rgba(0 ,0 ,0 ,0.5);">
                    <h1>Hello from WebView</h1>
                  </body>
                </html>"#
                    .to_owned(),
            ),
        })
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
