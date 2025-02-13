use crate::{egui::egui::Window, AssetStore, MenuResource};
use bevy::{asset::LoadedUntypedAsset, prelude::*};
use bevy_egui::EguiContexts;

#[derive(Resource)]
pub(crate) struct AssetsToLoad(Vec<Handle<LoadedUntypedAsset>>);
pub(crate) fn setup(assets: Res<AssetStore>, mut commands: Commands) {
    let assets_to_load: Vec<Handle<LoadedUntypedAsset>> =
        assets.asset_index.values().cloned().collect();
    commands.insert_resource(AssetsToLoad(assets_to_load));
}
pub(crate) fn run<T>(
    asset_server: Res<AssetServer>,
    mut to_load: ResMut<AssetsToLoad>,
    mut state: ResMut<NextState<T>>,
    mut egui_context: EguiContexts,
    menu_info: Res<MenuResource<T>>,
    mut store: ResMut<AssetStore>,
    loaded_assets: Res<crate::LoadedAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) where T: States,
{
    to_load.0.retain(|handle| {
        match asset_server.get_load_state(handle.id()) {
            Some(bevy::asset::LoadState::Loaded) => false,
            _ => true,
        }
    });
    if to_load.0.is_empty() {
        load_atlases(&mut store, &loaded_assets, &mut texture_atlases);
        state.set(menu_info.menu_state.clone());
    }
    Window::new("Loading, Please Wait").show(
        egui_context.ctx_mut(), |ui| {
            ui.label(
                format!("{} assets remaining", to_load.0.len())
            )
        });
}
fn load_atlases(
    store: &mut AssetStore,
    loaded_assets: &crate::LoadedAssets,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    for new_atlas in store.atlases_to_build.iter() {
        let texture_handle = store.get_handle(
            &new_atlas.texture_tag,
            loaded_assets
        ).unwrap();
        let atlas = TextureAtlas::from_grid(
            texture_handle,
            new_atlas.tile_size,
            new_atlas.sprites_x,
            new_atlas.sprites_y,
            None,
            None);
        let atlas_handle = texture_atlases.add(atlas);
        store
            .atlases
            .insert(new_atlas.tag.clone(), atlas_handle);
    }
}
pub(crate) fn exit(
    mut commands: Commands,
) {
    commands.remove_resource::<AssetsToLoad>();
}