#![feature(let_chains)]

use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
};

/// Optimizes levels by converting them to the postcard format, which is incredibly small and quick to deserialize at runtime
fn optimize_levels() {
    let levels_dir = fs::read_dir("levels").unwrap();

    let levels_path = PathBuf::from("assets/levels/");
    if !levels_path.exists() {
        fs::create_dir(&levels_path).unwrap();
    }

    for level in levels_dir {
        let level = level.unwrap();
        let path = level.path();

        // Check if is actually level file
        if path.ends_with(".toml") {
            println!("cargo:rerun-if-changed={}", level.path().display());

            let mut file = fs::File::open(&path).unwrap();

            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let level_definition: toml::Value = toml::from_str(&contents).unwrap();

            let postcard_definition = postcard::to_stdvec(&level_definition).unwrap();

            let destination_path = path.with_extension("bin");
            let level_name = destination_path.file_name().unwrap();
            let level_path = levels_path.join(level_name);

            let mut file = fs::File::create(level_path).unwrap();

            file.write_all(&postcard_definition).unwrap();
        }
    }
}

fn main() {
    // TODO: Use https://github.com/nlfiedler/magick-rust to auto-convert gimp files to png

    optimize_levels();
}
