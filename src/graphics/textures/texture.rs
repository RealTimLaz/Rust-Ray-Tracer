use crate::math::{Color, Point};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point) -> Color;
}