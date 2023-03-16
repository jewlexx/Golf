use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Level {
    #[serde(rename = "startingPosition")]
    pub starting_pos: Position,
    #[serde(rename = "goalPosition")]
    pub goal_pos: Position,
    pub element: Vec<Element>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum ElementShape {
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "triangle")]
    Triangle,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Position {
    #[serde(rename = "xOffset")]
    pub x_offset: f64,
    #[serde(rename = "yOffset")]
    pub y_offset: f64,
}
