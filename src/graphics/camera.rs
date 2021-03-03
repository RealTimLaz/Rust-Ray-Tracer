use std::f64::consts::PI;

use crate::math::{Point, Vec3};

use super::Ray;

pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    fn degrees_radians(deg: f64) -> f64 {
        deg * PI / 180.0
    }

    pub fn new<T>(position: Point, look_at: Vec3, up: Vec3, fov: T, aspect_ratio: f64) -> Camera
    where
        T: Into<f64> + Copy,
    {
        let theta = Camera::degrees_radians(fov.into());
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (position - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let origin = position;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
