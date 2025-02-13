use bevy::prelude::*;
use crate::bevy_assets::asset_store::*;
#[derive(Clone)]
pub enum AssetType {
    Image,
    Sound,
    SpriteSheet{tile_size: Vec2, sprites_x: usize, sprites_y: usize},
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
    fn asset_exists(filename: &str) -> anyhow::Result<()> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let current_directory = std::env::current_dir()?;
            let assets = current_directory.join("assets");
            let new_image = assets.join(filename);
            if !new_image.exists() {
                return Err(anyhow::Error::msg(format!(
                    "{} not found in assets directory",
                    &filename
                )));
            }
        }
        Ok(())
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
    pub fn add_sprite_sheet<S: ToString>(
        mut self,
        tag: S,
        filename: S,
        sprite_width: f32,
        sprite_height: f32,
        sprites_x: usize,
        sprites_y: usize,
    ) -> anyhow::Result<Self> {
        let filename = filename.to_string();
        AssetManager::asset_exists(&filename)?;
        self
            .asset_list
            .push((tag.to_string(), filename, AssetType::SpriteSheet{
                tile_size: Vec2::new(
                    sprite_width,
                    sprite_height),
                sprites_x,
                sprites_y,
            }));
        Ok(self)
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
        atlases_to_build: Vec::new(),
        atlases: bevy::utils::HashMap::new(),
    };
    asset_resource.asset_list.iter().for_each(
        |(tag, filename, asset_type)| {
            match asset_type {
                AssetType::SpriteSheet { tile_size, sprites_x, sprites_y } => {
                    // Sprite Sheets require that we load the image first, and defer
                    // sheet creation to the loading menu - after the image has loaded
                    let image_handle = asset_server.load_untyped(filename);//(4)
                    let base_tag = format!("{tag}_base");//(5)
                    assets
                        .asset_index
                        .insert(base_tag.clone(), image_handle);//(6)

                    // Now that its loaded, we store the future atlas in the asset store
                    assets.atlases_to_build.push(FutureAtlas {//(7)
                        tag: tag.clone(),
                        texture_tag: base_tag,
                        tile_size: *tile_size,
                        sprites_x: *sprites_x,
                        sprites_y: *sprites_y,
                    });
                }
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
