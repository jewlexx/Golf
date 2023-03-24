use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    utils::BoxedFuture,
};

use super::def::Level;

#[derive(Debug, Default)]
pub(crate) struct LevelLoader;

impl AssetLoader for LevelLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let level_asset: Level = postcard::from_bytes(bytes)?;
            let file_name = load_context
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();

            load_context
                .set_labeled_asset(&format!("level-{file_name}"), LoadedAsset::new(level_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["level"]
    }
}

#[derive(Debug, Clone, Resource)]
pub(crate) struct ActiveLevel {
    level: usize,
    data: Option<Handle<Level>>,
}

impl Default for ActiveLevel {
    fn default() -> Self {
        Self {
            // Default to 1 for now, as there is no level 0
            // In future level 0 will likely be a menu or something of that sort
            level: 1,
            data: None,
        }
    }
}

pub(crate) fn load_current(server: Res<AssetServer>, mut active: ResMut<ActiveLevel>) {
    if active.data.is_none() {
        let data = server.load::<Level, String>(format!("levels/{}.level", active.level));
        active.data = Some(data);
        println!("Loaded level {}", active.level);
    }
}

pub(crate) fn load_level(level: usize) {}
