use super::MenuResource;
use bevy::{app::AppExit, prelude::*};
use crate::{AssetStore};

#[derive(Component)]
pub(crate) struct MenuElement;

pub(crate) fn setup<T>(// (1)
                       state: Res<State<T>>,
                       mut commands: Commands,
                       menu_resource: Res<MenuResource<T>>,
                       loaded_assets: crate::AssetResource,
                       assets: Res<AssetStore>,
) where
    T: States,
{
    let current_state = state.get();// (2)
    let menu_graphic = match current_state {// (3)
        current_state if menu_resource.menu_state == *current_state =>
            assets.get_handle("main_menu", &loaded_assets).unwrap(),
        current_state if menu_resource.game_end_state == *current_state =>
            assets.get_handle("game_over", &loaded_assets).unwrap(),
        _ => panic!("Unknown menu state"),// (4)
    };

    commands
        .spawn(Camera2dBundle::default())// (5)
        .insert(MenuElement);
    commands
        .spawn(SpriteBundle {
            texture: menu_graphic,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(MenuElement);
}
pub(crate) fn run<T>(
    keyboard: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    current_state: Res<State<T>>,
    mut state: ResMut<NextState<T>>,
    menu_state: Res<MenuResource<T>>,
) where
    T: States,
{
    let current_state = current_state.get().clone();
    if current_state == menu_state.menu_state {
        if keyboard.just_pressed(KeyCode::P) {
            state.set(menu_state.game_start_state.clone());
        } else if keyboard.just_pressed(KeyCode::Q) {
            exit.send(AppExit);
        }
    }
    else if current_state == menu_state.game_end_state {
        if keyboard.just_pressed(KeyCode::M) {
            state.set(menu_state.menu_state.clone());
        } else if keyboard.just_pressed(KeyCode::Q) {
            exit.send(AppExit);
        }
    }
}
