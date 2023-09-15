use bevy::audio::PlaybackMode::Loop;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use std::ops::Add;

const BOUNDARY_X: f32 = 720.;
const BOUNDARY_Y: f32 = 1520.;

const MOVE_STEP: f32 = 15.;
const MOVE_COOL_DOWN: f32 = 0.1;

const DRAGON_SIZE: f32 = 50.;
const TREASURE_SIZE: f32 = 30.;

const DRAGON_REACH: f32 = 100.;

const DRAGON_START_POSITION: Vec3 = Vec3 {
    x: 0.,
    y: 0.,
    z: 2.,
};

const TREASURE_START_POSITION: Vec3 = Vec3 {
    x: -500.,
    y: -1000.,
    z: 1.,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update, (move_dragon, pick_up_treasure))
        .run();
}

#[derive(Default)]
struct Dragon {
    entity: Option<Entity>,
    position: Vec3,
    size: f32,
    reach: f32,
    move_cooldown: Timer,
}

#[derive(Default)]
struct Treasure {
    entity: Option<Entity>,
    position: Vec3,
    size: f32,
    moves_with: Option<Entity>,
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

    game.dragon.position = DRAGON_START_POSITION;
    game.dragon.size = DRAGON_SIZE;
    game.dragon.reach = DRAGON_REACH;
    game.dragon.move_cooldown = Timer::from_seconds(MOVE_COOL_DOWN, TimerMode::Once);

    game.treasure.position = TREASURE_START_POSITION;
    game.treasure.size = TREASURE_SIZE;

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
    let mut position_delta = Vec3::ZERO;

    if game.dragon.move_cooldown.tick(time.delta()).finished() {
        if keyboard_input.pressed(KeyCode::Up) {
            position_delta = position_delta.add(Vec3 {
                x: 0.,
                y: MOVE_STEP,
                z: 0.,
            });
        }
        if keyboard_input.pressed(KeyCode::Down) {
            position_delta = position_delta.add(Vec3 {
                x: 0.,
                y: -MOVE_STEP,
                z: 0.,
            });
        }
        if keyboard_input.pressed(KeyCode::Right) {
            position_delta = position_delta.add(Vec3 {
                x: MOVE_STEP,
                y: 0.,
                z: 0.,
            });
        }
        if keyboard_input.pressed(KeyCode::Left) {
            position_delta = position_delta.add(Vec3 {
                x: -MOVE_STEP,
                y: 0.,
                z: 0.,
            });
        }
    }

    let next_position = game.dragon.position.add(position_delta);
    
    if position_delta != Vec3::ZERO
        && next_position.x <= BOUNDARY_X
        && next_position.x >= -BOUNDARY_X
        && next_position.y <= BOUNDARY_Y
        && next_position.y >= -BOUNDARY_Y
    {
        game.dragon.move_cooldown.reset();

        game.dragon.position = next_position;

        game.treasure.position = match game.treasure.moves_with {
            None => game.treasure.position,
            Some(_entity) => game.treasure.position + position_delta,
        };

        *transforms.get_mut(game.landscape.entity.unwrap()).unwrap() =
            Transform::from_xyz(-game.dragon.position.x, -game.dragon.position.y, 0.);

        *transforms.get_mut(game.treasure.entity.unwrap()).unwrap() = Transform::from_xyz(
            game.treasure.position.x - game.dragon.position.x,
            game.treasure.position.y - game.dragon.position.y,
            game.treasure.position.z,
        );
    }
}

fn pick_up_treasure(keyboard_input: Res<Input<KeyCode>>, mut game: ResMut<Game>) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        game.treasure.moves_with = match game.treasure.moves_with {
            None => {
                let mut entity_to_move_with = None;
                if game
                    .dragon
                    .position
                    .xy()
                    .distance(game.treasure.position.xy())
                    <= game.dragon.reach
                {
                    entity_to_move_with = game.dragon.entity
                }
                entity_to_move_with
            }
            Some(_entity) => None,
        };
    }
}
