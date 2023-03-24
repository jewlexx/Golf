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

fn num_to_word(num: usize) -> String {
    match num {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
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
        let current_level = num_to_word(active.level);

        let data = server.load::<Level, String>(format!("levels/{current_level}.level"));
        active.data = Some(data);
        println!("Loaded level {}", current_level);
    }
}

pub(crate) fn load_level(level: usize) {}
