use crate::Vec3;

pub struct MaterialSample {
    pub base_color: Vec3,
    pub emission: Vec3,
}

pub trait Material {
    fn sample(&self, pos: Vec3) -> MaterialSample;
}

#[derive(Clone)]
pub struct BasicMaterial {
    pub base_color: Vec3,
    pub emission: Vec3,
}

impl Default for BasicMaterial {
    fn default() -> Self {
        Self {
            base_color: 0.9.into(),
            emission: 0.into(),
        }
    }
}

impl Material for BasicMaterial {
    fn sample(&self, _pos: Vec3) -> MaterialSample {
        MaterialSample {
            base_color: self.base_color,
            emission: self.emission,
        }
    }
}
