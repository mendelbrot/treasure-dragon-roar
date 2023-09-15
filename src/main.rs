use std::ops::Add;

use bevy::audio::PlaybackMode::Loop;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, move_dragon)
        .run();
}

#[derive(Default)]
struct Dragon {
    entity: Option<Entity>,
    position: Vec3,
    move_cooldown: Timer,
}

#[derive(Default)]
struct Treasure {
    entity: Option<Entity>,
    position: Vec3,
}

#[derive(Default)]
struct Landscape {
    entity: Option<Entity>,
}

#[derive(Resource, Default)]
struct Game {
    dragon: Dragon,
    treasure: Treasure,
    landscape: Landscape,
}

const BOUNDARY_X: f32 = 720.;
const BOUNDARY_Y: f32 = 1520.;

const MOVE_STEP: f32 = 15.;
const MOVE_COOL_DOWN: f32 = 0.1;

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

    game.dragon.position = Vec3 {
        x: 0.,
        y: 0.,
        z: 2.,
    };
    game.dragon.move_cooldown = Timer::from_seconds(MOVE_COOL_DOWN, TimerMode::Once);

    game.treasure.position = Vec3 {
        x: -500.,
        y: -1000.,
        z: 1.,
    };

    game.landscape.entity = Some(
        _commands
            .spawn(SpriteBundle {
                texture: asset_server.load("backgrounds/tall_landscape.png"),
                transform: Transform::from_xyz(
                    -game.dragon.position.x,
                    -game.dragon.position.y,
                    0.,
                ),
                ..default()
            })
            .id(),
    );

    game.treasure.entity = Some(
        _commands
            .spawn(SpriteBundle {
                texture: asset_server.load("objects2d/treasure_chest.png"),
                transform: Transform::from_xyz(
                    game.treasure.position.x - game.dragon.position.x,
                    game.treasure.position.y - game.dragon.position.y,
                    game.treasure.position.z,
                ),
                ..default()
            })
            .id(),
    );

    game.dragon.entity = Some(
        _commands
            .spawn(SpriteBundle {
                texture: asset_server.load("objects2d/dragon1.png"),
                transform: Transform::from_xyz(0., 0., game.dragon.position.z),
                ..default()
            })
            .id(),
    );
}

fn move_dragon(
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
) {
    if game.dragon.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;
        let mut next_position;

        if keyboard_input.pressed(KeyCode::Up) {
            next_position = game.dragon.position.add(Vec3 {
                x: 0.,
                y: MOVE_STEP,
                z: 0.,
            });
            if next_position.y <= BOUNDARY_Y {
                game.dragon.position = next_position
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            next_position = game.dragon.position.add(Vec3 {
                x: 0.,
                y: -MOVE_STEP,
                z: 0.,
            });
            if next_position.y >= -BOUNDARY_Y {
                game.dragon.position = next_position
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            next_position = game.dragon.position.add(Vec3 {
                x: MOVE_STEP,
                y: 0.,
                z: 0.,
            });
            if next_position.x <= BOUNDARY_X {
                game.dragon.position = next_position
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            next_position = game.dragon.position.add(Vec3 {
                x: -MOVE_STEP,
                y: 0.,
                z: 0.,
            });
            if next_position.x >= -BOUNDARY_X {
                game.dragon.position = next_position
            }
            moved = true;
        }

        // move on the board
        if moved {
            game.dragon.move_cooldown.reset();

            *transforms.get_mut(game.landscape.entity.unwrap()).unwrap() = 
                Transform::from_xyz(-game.dragon.position.x, -game.dragon.position.y, 0.);
            
            // relative move treasure
            *transforms.get_mut(game.treasure.entity.unwrap()).unwrap() =
                Transform::from_xyz(
                    game.treasure.position.x - game.dragon.position.x,
                    game.treasure.position.y - game.dragon.position.y,
                    game.treasure.position.z,
                );
        }
    }
}
