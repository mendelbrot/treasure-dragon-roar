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
    x: usize,
    y: usize,
    move_cooldown: Timer,
}

#[derive(Resource, Default)]
struct Game {
    dragon: Dragon,
}

const MAP_SIZE_X: usize = 21;
const MAP_SIZE_Y: usize = 21;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/windless_slopes.ogg"),
        ..default()
    });

    // spawn the dragon
    game.dragon.x = MAP_SIZE_X / 2;
    game.dragon.y = MAP_SIZE_Y / 2;
    game.dragon.move_cooldown = Timer::from_seconds(0.3, TimerMode::Once);
    game.dragon.entity = Some(
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("objects2d/dragon1_md.png"),
                transform: Transform::from_xyz(game.dragon.x as f32, game.dragon.y as f32, 0.),
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

        if keyboard_input.pressed(KeyCode::Up) {
            if game.dragon.y < MAP_SIZE_Y - 1 {
                game.dragon.y += 1;
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            if game.dragon.y > 0 {
                game.dragon.y -= 1;
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            if game.dragon.x < MAP_SIZE_X - 1 {
                game.dragon.x += 1;
            }
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            if game.dragon.x > 0 {
                game.dragon.x -= 1;
            }
            moved = true;
        }

        // move on the board
        if moved {
            game.dragon.move_cooldown.reset();
            *transforms.get_mut(game.dragon.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    game.dragon.x as f32,
                    game.dragon.y as f32,
                    0.
                ),
                ..default()
            };
        }
    }
}

// fn handle_action(time: Res<Time>, mut char_input_events: EventReader<ReceivedCharacter>) {
//     // timer.0.reset();
//     for event in char_input_events.iter() {
//         info!("{:?}: '{}'", event, event.char);
//     }
//     if timer.0.tick(time.delta()).just_finished() {
//         info!("Hello!");
//     }
// }
