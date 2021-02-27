mod hittable;
mod ray;
mod sphere;
mod vec3;

pub use self::ray::Ray;

pub use self::vec3::Vec3;
pub type Point3 = Vec3;
pub type Color = Vec3;

pub use self::sphere::Sphere;

pub use self::hittable::HitRecord;
pub use self::hittable::Hittable;
