use crate::{Material, MaterialSample, Vec3};

pub trait RenderedObject {
    fn distance_to(&self, point: Vec3) -> f32;
    fn material_at(&self, point: Vec3) -> MaterialSample;
}

pub trait Object: RenderedObject + Sized {
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

impl<T: RenderedObject + Sized> Object for T {}

pub struct Translated<T: RenderedObject> {
    object: T,
    translation: Vec3,
}

impl<T: RenderedObject> RenderedObject for Translated<T> {
    fn distance_to(&self, point: Vec3) -> f32 {
        self.object.distance_to(point - self.translation)
    }

    fn material_at(&self, point: Vec3) -> MaterialSample {
        self.object.material_at(point - self.translation)
    }
}

pub struct Scaled<T: RenderedObject> {
    object: T,
    scale: f32,
}

impl<T: RenderedObject> RenderedObject for Scaled<T> {
    fn distance_to(&self, point: Vec3) -> f32 {
        self.object.distance_to(point / self.scale) * self.scale
    }

    fn material_at(&self, point: Vec3) -> MaterialSample {
        self.object.material_at(point / self.scale)
    }
}

pub struct Sphere<M: Material> {
    mat: M,
}

pub fn sphere(mat: impl Material) -> Sphere<impl Material> {
    Sphere { mat }
}

impl<M: Material> RenderedObject for Sphere<M> {
    fn distance_to(&self, point: Vec3) -> f32 {
        point.magnitude() - 1.0
    }

    fn material_at(&self, point: Vec3) -> MaterialSample {
        self.mat.sample(point)
    }
}

pub struct Cube<M: Material> {
    size: Vec3,
    mat: M,
}

pub fn cube<M: Material, T: Into<Vec3>>(mat: M, size: T) -> Cube<M> {
    Cube {
        mat,
        size: size.into(),
    }
}

impl<M: Material> RenderedObject for Cube<M> {
    fn distance_to(&self, point: Vec3) -> f32 {
        let face_distances = point.abs() - self.size;
        face_distances.x.max(face_distances.y.max(face_distances.z))
    }

    fn material_at(&self, point: Vec3) -> MaterialSample {
        self.mat.sample(point)
    }
}
