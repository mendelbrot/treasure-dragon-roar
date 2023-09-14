use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins)
    .add_systems(Startup, setup).run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("characters/dragon1_md.png"),
        ..default()
    });
}