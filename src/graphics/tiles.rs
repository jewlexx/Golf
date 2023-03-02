use bevy::prelude::*;
use bevy_tileset::prelude::*;

#[derive(Debug, Resource)]
pub struct Tiles {
    pub grass: Handle<Tileset>,
}

pub fn load(asset_server: Res<AssetServer>, mut tiles: ResMut<Tiles>) {
    tiles.grass = asset_server.load("tiles/grass.ron");
}
