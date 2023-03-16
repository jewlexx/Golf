pub mod def;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_toml_demo() {
        let demo = include_str!("../../levels/demo.toml");

        let level: def::Level = toml::from_str(demo).unwrap();

        dbg!(level);
    }

    #[test]
    fn test_parse_postcard_demo() {
        let demo = include_bytes!("../../assets/levels/demo.bin");

        let level: def::Level = postcard::from_bytes(demo).unwrap();

        dbg!(level);
    }
}
