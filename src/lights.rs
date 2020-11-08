use crate::{Vec3, MAX_SDF_DISTANCE};

pub trait ImmediateLight {
    fn sample(&self, from: Vec3) -> LightSample;
}

pub struct LightSample {
    pub shadow_ray_target: Vec3,
    pub color: Vec3,
}

#[derive(Clone, Debug)]
pub struct DirectionalLight {
    direction: Vec3,
    size: f32,
    color: Vec3,
}

impl DirectionalLight {
    pub fn new<T: Into<Vec3>>(direction: T, size: f32, color: Vec3) -> Self {
        Self {
            direction: direction.into().normalized(),
            size,
            color,
        }
    }
}

impl ImmediateLight for DirectionalLight {
    fn sample(&self, from: Vec3) -> LightSample {
        // TODO: Size.
        LightSample {
            shadow_ray_target: from - self.direction * MAX_SDF_DISTANCE,
            color: self.color,
        }
    }
}
