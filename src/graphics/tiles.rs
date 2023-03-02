use bevy::prelude::*;
use bevy_tileset::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Tiles {
    pub grass: Option<Handle<Tileset>>,
}

pub fn load(asset_server: Res<AssetServer>, mut tiles: ResMut<Tiles>) {
    tiles.grass = Some(asset_server.load("tiles/grass.ron"));
}
