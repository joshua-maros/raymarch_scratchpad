use crate::Vec3;

/// Implemented on objects that can be rendered.
pub trait SdfObject {
    fn distance_to(&self, point: Vec3) -> f32;
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
}

impl SdfObject for Sphere {
    fn distance_to(&self, point: Vec3) -> f32 {
        self.origin.distance(point) - self.radius
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    pub origin: Vec3,
    pub size: Vec3,
}

impl SdfObject for Cube {
    fn distance_to(&self, point: Vec3) -> f32 {
        let face_distances = (point - self.origin).abs() - self.size;
        face_distances.x.max(face_distances.y.max(face_distances.z))
    }
}
