mod material;

mod dielectric;
mod lambertian;
mod metal;

pub use material::Material;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
