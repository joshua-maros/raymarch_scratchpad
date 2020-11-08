use crate::Vec3;
use std::fmt::Debug;

pub trait RenderedObject {
    fn distance_to(&self, point: Vec3) -> f32;
}

pub trait Object: Sized + Clone + Debug {
    fn translated<V: Into<Vec3>>(self, offset: V) -> Translated<Self> {
        Translated {
            object: self,
            translation: offset.into(),
        }
    }

    fn scaled<V: num_traits::NumCast>(self, scale: V) -> Scaled<Self> {
        Scaled {
            object: self,
            scale: num_traits::cast(scale).unwrap(),
        }
    }
}

impl<T: RenderedObject + Sized + Clone + Debug> Object for T {}

#[derive(Clone, Debug)]
pub struct Translated<T: Object + Clone + Debug> {
    object: T,
    translation: Vec3,
}

impl<T: RenderedObject + Clone + Debug> RenderedObject for Translated<T> {
    fn distance_to(&self, point: Vec3) -> f32 {
        self.object.distance_to(point - self.translation)
    }
}

#[derive(Clone, Debug)]
pub struct Scaled<T: Object + Clone + Debug> {
    object: T,
    scale: f32,
}

impl<T: RenderedObject + Clone + Debug> RenderedObject for Scaled<T> {
    fn distance_to(&self, point: Vec3) -> f32 {
        self.object.distance_to(point / self.scale) * self.scale
    }
}

#[derive(Clone, Debug)]
pub struct Sphere;

pub fn sphere() -> Sphere {
    Sphere
}

impl RenderedObject for Sphere {
    fn distance_to(&self, point: Vec3) -> f32 {
        point.magnitude() - 1.0
    }
}

#[derive(Clone, Debug)]
pub struct Cube {
    size: Vec3,
}

pub fn cube<T: Into<Vec3>>(size: T) -> Cube {
    Cube { size: size.into() }
}

impl RenderedObject for Cube {
    fn distance_to(&self, point: Vec3) -> f32 {
        let face_distances = point.abs() - self.size;
        face_distances.x.max(face_distances.y.max(face_distances.z))
    }
}
