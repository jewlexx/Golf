use bevy::prelude::*;

use crate::components::{ball::Ball, vel::Velocity};

pub(crate) mod def;

pub(crate) fn reset_level(
    mut commands: Commands,
    elements: Query<&mut def::Element>,
    mut query: Query<(&mut Transform, &mut Ball, &mut Velocity)>,
) {
    todo!("Implement resetting of levels")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_postcard() -> def::Level {
        let demo = include_bytes!("../../assets/levels/demo.bin");

        postcard::from_bytes(demo).unwrap()
    }

    fn parse_toml() -> def::Level {
        let demo = include_str!("../../levels/demo.toml");

        toml::from_str(demo).unwrap()
    }

    #[test]
    fn test_parse_toml_demo() {
        _ = parse_toml();
    }

    #[test]
    fn test_parse_postcard_demo() {
        _ = parse_postcard();
    }

    #[test]
    fn test_ensure_consistency() {
        assert_eq!(parse_toml(), parse_postcard());
    }
}
