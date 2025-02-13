use bevy::prelude::*;
use my_library::*;

#[derive(Component)]
struct Flappy {
  gravity: f32,
}

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct FlappyElement;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
  #[default] Loading,
  MainMenu,
  Flapping,
  GameOver
}

fn main() -> anyhow::Result<()> {
  let mut app = App::new();

  add_phase!(app, GamePhase, GamePhase::Flapping,
    start => [ setup ],
    run => [ gravity, flap, clamp, move_walls, hit_wall, cycle_animations ],
    exit => [ cleanup::<FlappyElement> ]
  );

  app.add_plugins(DefaultPlugins.set(WindowPlugin {
    primary_window: Some(Window {
      title: "Flappy Dragon - Bevy Edition".to_string(),
      resolution: bevy::window::WindowResolution::new(
        1024.0, 768.0
      ),
      ..default()
    }),
    ..default()
  }))
      .add_plugins(RandomPlugin)
      .add_plugins(GameStatePlugin::new(
        GamePhase::MainMenu,
        GamePhase::Flapping,
        GamePhase::GameOver)
      )
      .add_plugins(AssetManager::new()
          .add_image("dragon", "flappy_dragon.png")?
          .add_image("wall", "wall.png")?
          .add_sprite_sheet("flappy", "flappy_sprite_sheet.png",
                            62.0, 65.0, 4, 1)?
      )
      .insert_resource(
        Animations::new()
            .with_animation("Straight and Level", PerFrameAnimation::new(
              vec![
                AnimationFrame::new(2, 500, vec![AnimationOption::NextFrame]),
                AnimationFrame::new(3, 500, vec![AnimationOption::GoToFrame(0)]),
              ]
            ))
            .with_animation("Flapping", PerFrameAnimation::new(
              vec![
                AnimationFrame::new(0, 66, vec![AnimationOption::NextFrame,
                                                AnimationOption::PlaySound("flap".to_string())]),
                AnimationFrame::new(1, 66, vec![AnimationOption::NextFrame]),
                AnimationFrame::new(2, 66, vec![AnimationOption::NextFrame]),
                AnimationFrame::new(3, 66, vec![AnimationOption::NextFrame]),
                AnimationFrame::new(2, 66, vec![AnimationOption::NextFrame]),
                AnimationFrame::new(1, 66, vec![
                  AnimationOption::SwitchToAnimation(
                    "Straight and Level".to_string())
                ]),
              ]
            ))
      )
      .run();
  
  Ok(())
}

fn setup(
  mut commands: Commands,
  mut rng: ResMut<RandomNumberGenerator>,
  assets: Res<AssetStore>,
  loaded_assets: AssetResource,
) {
  commands.spawn(Camera2dBundle::default()).insert(FlappyElement);
  spawn_animated_sprite!(assets, commands, "flappy", -490.0, 0.0, 10.0,
    "Straight and Level", Flappy { gravity: 0.0 }, FlappyElement);
  build_wall(&mut commands, &assets, &loaded_assets, rng.range(-5..5));
}

fn build_wall(
  commands: &mut Commands,
  assets: &AssetStore,
  loaded_assets: &LoadedAssets,
  gap_y: i32,
) {
  for y in -12..=12 {
    if y < gap_y - 4 || y > gap_y + 4 {
      spawn_image!(assets, commands, "wall", 512.0, y as f32 * 32.0, 1.0,
        &loaded_assets, Obstacle, FlappyElement);
    }
  }
}

fn gravity(mut query: Query<(&mut Flappy, &mut Transform)>) {
  if let Ok((mut flappy, mut transform)) = query.get_single_mut() {
    flappy.gravity += 0.1;
    transform.translation.y -= flappy.gravity;
  }
}

fn flap(
  keyboard: Res<Input<KeyCode>>,
  mut query: Query<(&mut Flappy, &mut AnimationCycle)>
)
{
  if keyboard.pressed(KeyCode::Space) {
    if let Ok((mut flappy, mut animation)) = query.get_single_mut() {
      flappy.gravity = -5.0;
      animation.switch("Flapping");
    }
  }
}

fn clamp(
  mut query: Query<&mut Transform, With<Flappy>>,
  mut state: ResMut<NextState<GamePhase>>,
) {
  if let Ok(mut transform) = query.get_single_mut() {
    if transform.translation.y > 384.0 {
      transform.translation.y = 384.0;
    } else if transform.translation.y < -384.0 {
      state.set(GamePhase::GameOver);
    }
  }
}

fn move_walls(
  mut commands: Commands,
  mut query: Query<&mut Transform, With<Obstacle>>,
  delete: Query<Entity, With<Obstacle>>,
  assets: Res<AssetStore>,
  loaded_assets: Res<LoadedAssets>,//&LoadedAssets,
  mut rng: ResMut<RandomNumberGenerator>,
) {
  let mut rebuild = false;
  for mut transform in query.iter_mut() {
    transform.translation.x -= 4.0;
    if transform.translation.x < -530.0 {
      rebuild = true;
    }
  }

  if rebuild {
    for entity in delete.iter() {
      commands.entity(entity).despawn();
    }
    build_wall(&mut commands, &assets, &loaded_assets,rng.range(-5..5),);
  }
}

fn hit_wall(
  player: Query<&Transform, With<Flappy>>,
  walls: Query<&Transform, With<Obstacle>>,
  mut state: ResMut<NextState<GamePhase>>,
) {
  if let Ok(player) = player.get_single() {
    for wall in walls.iter() {
      let distance = player.translation.distance(wall.translation);
      if distance < 32.0 {
        state.set(GamePhase::GameOver);
      }
    }
  }
}