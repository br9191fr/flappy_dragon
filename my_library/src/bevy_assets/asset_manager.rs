use bevy::prelude::*;
use crate::bevy_assets::asset_store::*;
#[derive(Clone)]
pub enum AssetType {
    Image,
}

#[derive(Resource, Clone)]
pub struct AssetManager {
    asset_list: Vec<(String, String, AssetType)>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            asset_list: vec![
                ("main_menu".to_string(), "main_menu.png".to_string(),
                AssetType::Image),
                ("game_over".to_string(), "game_over.png".to_string(),
                AssetType::Image),
            ],
        }
    }

    pub fn add_image<S: ToString>(//(3)
                                  mut self,
                                  tag: S,
                                  filename: S,
    ) -> anyhow::Result<Self> {//(4)
        let filename = filename.to_string();//(5)
        #[cfg(not(target_arch = "wasm32"))]//(6)
        {
            let current_directory = std::env::current_dir()?;//(7)
            //println!("dir = {}",current_directory.to_str().unwrap());
            let assets = current_directory.join("assets");//(8)
            let new_image = assets.join(&filename);
            if !new_image.exists() {
                return Err(anyhow::Error::msg(format!(//(9)
                                                      "{} not found in assets directory",
                                                      &filename
                )));
            }
        }
        self//(10)
            .asset_list
            .push((tag.to_string(), filename, AssetType::Image));
        Ok(self)//(11)
    }
}

impl Plugin for AssetManager {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.clone());
        app.add_systems(Startup, setup);
    }
}

fn setup(
    asset_resource: Res<AssetManager>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut assets = AssetStore {
        asset_index: bevy::utils::HashMap::new(),
    };
    asset_resource.asset_list.iter().for_each(
        |(tag, filename, asset_type)| {
            match asset_type {
                _ => {
                    // Most asset types don't require a separate loader
                    assets
                        .asset_index
                        .insert(tag.clone(), asset_server.load_untyped(filename));
                }
            }
        },
    );
    commands.remove_resource::<AssetManager>();
    commands.insert_resource(assets);
}
