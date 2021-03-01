use crate::math::{Point, Vec3};
pub struct Ray<'a> {
    pub origin: &'a Point,
    pub direction: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point, direction: &'a Vec3) -> Self {
        Ray { origin, direction }
    }
}
