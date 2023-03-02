use bevy::prelude::*;
use bevy_tileset::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Background {
    pub handle: Option<Handle<Tileset>>,
}

pub fn load(mut background_tileset: ResMut<Background>, asset_server: Res<AssetServer>) {
    println!("Loading tileset");
    background_tileset.handle = Some(asset_server.load("tilesets/background.ron"));
    println!(
        "Loaded tileset with handle: {:?}",
        background_tileset.handle
    );
}

pub fn show(
    tilesets: Tilesets,
    mut commands: Commands,
    tileset: Res<Background>,
    mut has_ran: Local<bool>,
) {
    if tileset.handle.is_none() || *has_ran || !tilesets.contains_id(&0) {
        return;
    }

    if let Some(tileset) = tilesets.get_by_name("Background") {
        let atlas = tileset.atlas();

        if let Some((TileIndex::Standard(index), ..)) = tileset.select_tile("Grass") {
            // Do something standard
            commands.spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(08.0, -48.0, 0.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(index),
                texture_atlas: atlas.clone(),
                ..Default::default()
            });
        }

        *has_ran = true;
    }
}
