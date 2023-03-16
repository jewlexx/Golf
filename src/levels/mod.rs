use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Level {
    #[serde(rename = "startingPosition")]
    pub starting_pos: Position,
    #[serde(rename = "goalPosition")]
    pub goal_pos: Position,
    pub element: Element,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ElementType {
    pub shape: ElementShape,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ElementShape {
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "triangle")]
    Triangle,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Element {
    pub shape: ElementType,
    #[serde(rename = "xOffset")]
    pub x_offset: f64,
    #[serde(rename = "yOffset")]
    pub y_offset: f64,
    #[serde(rename = "width")]
    pub width: String,
    #[serde(rename = "height")]
    pub height: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    #[serde(rename = "xOffset")]
    pub x_offset: f64,
    #[serde(rename = "yOffset")]
    pub y_offset: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LevelOptions {
    #[serde(rename = "level")]
    Level(Level),
    #[serde(rename = "startingPosition")]
    StartingPos(Position),
    #[serde(rename = "goalPosition")]
    EndingPos(Position),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LevelDefinition {
    level: LevelOptions,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_all() {
        let demo_level = include_str!("../../levels/demo.toml");
        let level: Level = toml::de::from_str(demo_level).unwrap();

        println!("{level:#?}");
    }
}
