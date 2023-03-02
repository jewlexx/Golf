use bevy::prelude::*;
use bevy_tileset::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Tiles {
    pub handle: Option<Handle<Tileset>>,
}

pub fn load(asset_server: Res<AssetServer>, mut tiles: ResMut<Tiles>) {
    tiles.handle = Some(asset_server.load("tilesets/background.ron"));
}

pub fn show(tilesets: Tilesets, mut commands: Commands, tileset: Res<Tiles>) {
    if tileset.handle.is_none() || !tilesets.contains_name("Background") {
        println!("No grass tileset!");
        return;
    }

    if let Some(tileset) = tilesets.get_by_name("Background") {
        let atlas = tileset.atlas();
        let texture = tileset.texture().clone();

        dbg!(&texture);

        commands.spawn(SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        });

        if let Some((ref tile_index, ..)) = tileset.select_tile("Grass") {
            dbg!(&tile_index);
            match tile_index {
                TileIndex::Standard(index) => {
                    dbg!(&index);
                    // Do something standard
                    commands.spawn(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(08.0, -48.0, 0.0),
                            ..Default::default()
                        },
                        sprite: TextureAtlasSprite::new(*index),
                        texture_atlas: atlas.clone(),
                        ..Default::default()
                    });
                }
                TileIndex::Animated(start, end, speed) => {
                    dbg!("Animated tileset wtf");

                    // Do something  ✨ animated ✨
                }
            }
        }
    }
}
