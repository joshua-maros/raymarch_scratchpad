use crate::Vec3;
use rand::Rng;

pub const MAX_SDF_DISTANCE: f32 = 1e5;
pub const MIN_SDF_DISTANCE: f32 = 1e-5;

pub fn random_bounce_direction(normal: Vec3) -> Vec3 {
    let mut rng = rand::thread_rng();
    let random_point_on_sphere = loop {
        let random_vec = Vec3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        // Throw away all vectors with length greater than one.
        if random_vec.magnitude() > 1.0 {
            continue;
        }
        break random_vec.normalized();
    };
    (random_point_on_sphere + normal).normalized()
}
