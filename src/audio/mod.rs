use bevy::prelude::*;

#[derive(Debug, Clone, Default, Resource)]
pub(crate) struct Sfx {
    pub(crate) hit: Option<Handle<AudioSource>>,
}

impl Sfx {
    pub(crate) fn init(server: Res<AssetServer>, mut sfx: ResMut<Self>) {
        sfx.hit = Some(server.load("audio/ball hit.wav"));
    }
}
