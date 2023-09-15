use std::ops::Add;
use std::ops::Mul;

use bevy::audio::PlaybackMode::Loop;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, gizmo_shapes)
        .add_systems(Update, move_dragon)
        .run();
}

#[derive(Default)]
struct Dragon {
    entity: Option<Entity>,
    position_unscaled: Vec3,
    move_cooldown: Timer,
}

#[derive(Resource, Default)]
struct Game {
    dragon: Dragon,
}

const MAP_RADIUS: f32 = 30.;
const MAP_SCALE: f32 = 15.;

const COOL_DOWN: f32 = 0.1;

fn setup(mut _commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    _commands.spawn(Camera2dBundle::default());
    _commands.spawn(AudioBundle {
        source: asset_server.load("sounds/windless_slopes.ogg"),
        settings: PlaybackSettings {
            mode: Loop,
            ..default()
        },
        ..default()
    });

    // spawn the dragon
    game.dragon.position_unscaled = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    game.dragon.move_cooldown = Timer::from_seconds(COOL_DOWN, TimerMode::Once);
    game.dragon.entity = Some(
        _commands
            .spawn(SpriteBundle {
                texture: asset_server.load("objects2d/dragon1_md.png"),
                transform: Transform::from_translation(
                    game.dragon.position_unscaled.mul(MAP_SCALE),
                ),
                ..default()
            })
            .id(),
    );
}

fn move_dragon(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.dragon.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        let mut next_position_unscaled = game.dragon.position_unscaled;

        if keyboard_input.pressed(KeyCode::Up) {
            next_position_unscaled = game.dragon.position_unscaled.add(Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            });
            if next_position_unscaled.distance(Vec3::ZERO) <= MAP_RADIUS {
                game.dragon.position_unscaled = next_position_unscaled
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            next_position_unscaled = game.dragon.position_unscaled.add(Vec3 {
                x: 0.,
                y: -1.,
                z: 0.,
            });
            if next_position_unscaled.distance(Vec3::ZERO) <= MAP_RADIUS {
                game.dragon.position_unscaled = next_position_unscaled
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            next_position_unscaled = game.dragon.position_unscaled.add(Vec3 {
                x: 1.,
                y: 0.,
                z: 0.,
            });
            if next_position_unscaled.distance(Vec3::ZERO) <= MAP_RADIUS {
                game.dragon.position_unscaled = next_position_unscaled
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            next_position_unscaled = game.dragon.position_unscaled.add(Vec3 {
                x: -1.,
                y: 0.,
                z: 0.,
            });
            if next_position_unscaled.distance(Vec3::ZERO) <= MAP_RADIUS {
                game.dragon.position_unscaled = next_position_unscaled
            }
            moved = true;
        }

        // move on the board
        if moved {
            game.dragon.move_cooldown.reset();
            *transforms.get_mut(game.dragon.entity.unwrap()).unwrap() =
                Transform::from_translation(game.dragon.position_unscaled.mul(MAP_SCALE))
        }
    }
}

fn gizmo_shapes(mut gizmos: Gizmos) {
    // debug shapes
    gizmos.circle_2d(Vec2::ZERO, (MAP_RADIUS * MAP_SCALE) as f32, Color::BLUE);
    gizmos.circle_2d(Vec2::ZERO, 5., Color::GREEN);
}
