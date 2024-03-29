use bevy::prelude::*;
use bevy_tileset::prelude::*;

#[derive(Debug, Default, Resource)]
pub(crate) struct Background {
    pub(crate) handle: Option<Handle<Tileset>>,
}

pub(crate) fn load(asset_server: Res<AssetServer>, mut background_tileset: ResMut<Background>) {
    println!("Loading tileset");
    background_tileset.handle = Some(asset_server.load("tilesets/background.ron"));
    println!(
        "Loaded tileset with handle: {:?}",
        background_tileset.handle
    );
}

pub(crate) fn show(
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
            // This relies on the tiles being 48x48
            let x_tiles = (crate::SCREEN_WIDTH / 48.) as i32 + 1;
            let y_tiles = (crate::SCREEN_HEIGHT / 48.) as i32 + 1;

            for y_index in 0..y_tiles {
                let tile_y_offset = (y_index * 48 - 300 + 12) as f32;

                for x_index in 0..x_tiles {
                    let tile_x_offset = (x_index * 48 - 450 + 12) as f32;

                    commands.spawn(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(tile_x_offset, tile_y_offset, 0.),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(index),
                        texture_atlas: atlas.clone(),
                        ..Default::default()
                    });
                }
            }
        }

        *has_ran = true;
    }
}
