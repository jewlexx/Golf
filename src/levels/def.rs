use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Level {
    #[serde(rename = "startingPosition")]
    pub starting_pos: Position,
    #[serde(rename = "goalPosition")]
    pub goal_pos: Position,
    pub element: Vec<Element>,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum ElementShape {
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "triangle")]
    Triangle,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Element {
    pub shape: ElementShape,

    #[serde(rename = "xOffset")]
    pub x_offset: f64,
    #[serde(rename = "yOffset")]
    pub y_offset: f64,

    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct Position {
    #[serde(rename = "xOffset")]
    pub x_offset: f64,
    #[serde(rename = "yOffset")]
    pub y_offset: f64,
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
