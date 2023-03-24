use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    utils::BoxedFuture,
};

use super::def::Level;

fn word_to_num(word: String) -> usize {
    // Only supports up to
    match word.as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "ten" => 10,
        "eleven" => 11,
        "twelve" => 12,
        "thirteen" => 13,
        "fourteen" => 14,
        "fifteen" => 15,
        "sixteen" => 16,
        "seventeen" => 17,
        "eighteen" => 18,
        "nineteen" => 19,
        _ => unimplemented!(),
    }
}

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

            let level_number = word_to_num(file_name);

            load_context.set_labeled_asset(
                &format!("level-{level_number}"),
                LoadedAsset::new(level_asset),
            );
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
    data: Option<Level>,
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

pub(crate) fn load_current(server: Res<AssetServer>, active: ResMut<ActiveLevel>) {}

pub(crate) fn load_level(level: usize) {}
