use crate::{SdfObject, Vec3};
use rand::Rng;

pub struct Scene {
    objects: Vec<Box<dyn SdfObject>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add_object<T: SdfObject + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }

    fn distance_field_at(&self, point: Vec3) -> f32 {
        let mut value = std::f32::MAX;
        for obj in &self.objects {
            value = value.min(obj.distance_to(point));
        }
        value
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        const EPSILON: f32 = 1e-4;
        let df_center = self.distance_field_at(point);
        let df_x = self.distance_field_at(point + (EPSILON, 0, 0));
        let df_y = self.distance_field_at(point + (0, EPSILON, 0));
        let df_z = self.distance_field_at(point + (0, 0, EPSILON));
        Vec3::from((df_x - df_center, df_y - df_center, df_z - df_center)).normalized()
    }
}

fn random_bounce_direction(normal: Vec3) -> Vec3 {
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
