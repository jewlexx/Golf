use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Component)]
pub(crate) struct Level {
    #[serde(rename = "startingPosition")]
    pub(crate) starting_pos: Position,
    #[serde(rename = "goalPosition")]
    pub(crate) goal_pos: Position,
    #[serde(default)]
    pub(crate) element: Vec<Element>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Component)]
pub(crate) enum ElementShape {
    #[serde(rename = "square")]
    Square,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "triangle")]
    Triangle,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Component)]
pub(crate) struct Element {
    pub(crate) shape: ElementShape,

    pub(crate) width: f64,
    pub(crate) height: f64,

    pub(crate) position: Position,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Component)]
pub(crate) struct Position {
    #[serde(rename = "xOffset")]
    pub(crate) x_offset: f64,
    #[serde(rename = "yOffset")]
    pub(crate) y_offset: f64,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Resource)]
pub(crate) struct BallStartingPosition {
    pub(crate) pos: Position,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Resource)]
pub(crate) struct GoalPosition {
    pub(crate) pos: Position,
}
