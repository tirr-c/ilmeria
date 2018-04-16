pub mod shape;

pub use self::shape::{RawShape, ProcessedShape};

use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}

#[derive(Clone, Debug)]
pub struct Ingredient {
    color: Color,
    shape: HashSet<ProcessedShape>,
}

impl Ingredient {
    pub fn from_raw_shape(color: Color, raw_shape: RawShape) -> Self {
        Ingredient {
            color,
            shape: raw_shape.available_shapes(),
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn shape(&self) -> &HashSet<ProcessedShape> {
        &self.shape
    }
}
