use std::f64::consts::PI;
use rand::Rng;

use crate::math::{Point, Vec3};

use super::Ray;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    fn degrees_radians(deg: f64) -> f64 {
        deg * PI / 180.0
    }

    pub fn new<T>(
        position: Point,
        look_at: Vec3,
        up: Vec3,
        fov: T,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Camera
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
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;

        let mut rng = rand::thread_rng();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rng.gen_range(self.time0..self.time1),
        )
    }
}
