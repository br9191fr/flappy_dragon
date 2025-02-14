use bevy::prelude::*;
use my_library::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
  #[default]
  Loading,
  MainMenu,
  Playing,
  GameOver,
}

#[derive(Component)]
struct GameElement;

#[derive(Component)]
struct Player;

fn main() -> anyhow::Result<()> {
  let mut app = App::new();
  add_phase!(app, GamePhase, GamePhase::Playing,
    start => [ setup ],
    run => [ movement, end_game, physics_clock, sum_impulses, apply_gravity, 
      apply_velocity, terminal_velocity ],
    exit => [ cleanup::<GameElement> ]
  );

  app.add_event::<Impulse>();
  app.add_event::<PhysicsTick>();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Mars Base One".to_string(),
        resolution: bevy::window::WindowResolution::new(1024.0, 768.0),
        ..default()
      }),
      ..default()
    }))
    .add_plugins(RandomPlugin)
    .add_plugins(GameStatePlugin::new(
      GamePhase::MainMenu,
      GamePhase::Playing,
      GamePhase::GameOver,
    ))
    .add_plugins(AssetManager::new().add_image("ship", "ship.png")?)
    .insert_resource(Animations::new())
    .run();

  Ok(())
}

fn setup(
  mut commands: Commands,
  assets: Res<AssetStore>,
  loaded_assets: Res<LoadedAssets>,
) {
  commands
    .spawn(Camera2dBundle::default())
    .insert(GameElement);
  spawn_image!(
    assets,
    commands,
    "ship",
    0.0,
    0.0,
    1.0,
    &loaded_assets,
    GameElement,
    Player,
    Velocity::default(),
    PhysicsPosition::new(Vec2::new(0.0, 0.0)),
    ApplyGravity(0.2)
  );
}

fn end_game(
  mut state: ResMut<NextState<GamePhase>>,
  player_query: Query<&Transform, With<Player>>,
) {
  let transform = player_query.single();
  if transform.translation.y < -384.0 || transform.translation.y > 384.0 ||
      transform.translation.x < -512.0 || transform.translation.x > 512.0
  {
    state.set(GamePhase::GameOver);
  }
}
fn movement(
  keyboard: Res<Input<KeyCode>>,
  mut player_query: Query<(Entity, &mut Transform), With<Player>>,
  mut impulses: EventWriter<Impulse>,
) {
  let (entity, mut transform) = player_query.single_mut();
  if keyboard.pressed(KeyCode::Left) {
    transform.rotate(Quat::from_rotation_z(f32::to_radians(2.0)));
  }
  if keyboard.pressed(KeyCode::Right) {
    transform.rotate(Quat::from_rotation_z(f32::to_radians(-2.0)));
  }
  if keyboard.pressed(KeyCode::Up) {
    impulses.send(Impulse {
      target: entity,
      amount: transform.local_y() / 5.0,
      absolute: false,
    });
  }
}
fn terminal_velocity(mut player_query: Query<&mut Velocity, With<Player>>) {
  let mut velocity = player_query.single_mut();
  let v2 = velocity.0.truncate();
  if v2.length() > 5.0 {
    let v2 = v2.normalize() * 5.0;
    velocity.0.x = v2.x;
    velocity.0.y = v2.y;
  }
}