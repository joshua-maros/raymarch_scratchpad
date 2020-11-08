use crate::{ImmediateLight, RenderedObject, Vec3, MAX_SDF_DISTANCE, MIN_SDF_DISTANCE};

pub struct Scene {
    objects: Vec<Box<dyn RenderedObject>>,
    lights: Vec<Box<dyn ImmediateLight>>,
    sky_color: Vec3,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            lights: vec![],
            sky_color: (0, 0, 1).into(),
        }
    }

    pub fn add_object<T: RenderedObject + 'static>(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }

    pub fn add_light<T: ImmediateLight + 'static>(&mut self, obj: T) {
        self.lights.push(Box::new(obj));
    }

    pub fn set_sky_color<T: Into<Vec3>>(&mut self, color: T) {
        self.sky_color = color.into();
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

    fn march_can_reach(&self, ray_start: Vec3, target: Vec3) -> bool {
        let mut remaining_len = (target - ray_start).magnitude();
        let dir = (target - ray_start).normalized();
        let mut pos = ray_start;
        loop {
            let df = self.distance_field_at(pos);
            if df <= MIN_SDF_DISTANCE {
                return false;
            }
            remaining_len -= df;
            if remaining_len <= 0.0 {
                return true;
            }
            pos += dir * df;
        }
    }

    fn march_until_hit(&self, ray_start: Vec3, ray_dir: Vec3) -> Option<Vec3> {
        let mut pos = ray_start;
        loop {
            let df = self.distance_field_at(pos);
            if df <= MIN_SDF_DISTANCE {
                return Some(pos);
            } else if df >= MAX_SDF_DISTANCE {
                return None;
            }
            pos += ray_dir * df;
        }
    }

    fn color_on_surface(&self, surface_pos: Vec3, remaining_bounces: u32) -> Vec3 {
        let mut result: Vec3 = 0.into();
        let normal = self.normal_at(surface_pos);
        let ray_start = surface_pos + normal * MIN_SDF_DISTANCE * 2.0;
        let mat = self
            .objects
            .iter()
            .find(|o| o.distance_to(surface_pos) <= MIN_SDF_DISTANCE)
            .unwrap()
            .material_at(surface_pos);
        let surface_color = mat.base_color;
        for light in &self.lights {
            let sample = light.sample(ray_start);
            let brightness = (sample.shadow_ray_target - ray_start)
                .normalized()
                .dot(normal);
            if brightness > 0.0 && self.march_can_reach(ray_start, sample.shadow_ray_target) {
                result += sample.color * surface_color * brightness;
            }
        }
        if remaining_bounces > 0 {
            let dir = (Vec3::random_unit_vec() + normal).normalized();
            // This should never be negative.
            let brightness = dir.dot(normal);
            let light = self.do_camera_ray(ray_start, dir, remaining_bounces - 1);
            result += light * surface_color * brightness;
        }
        result
    }

    pub fn do_camera_ray(&self, origin: Vec3, direction: Vec3, remaining_bounces: u32) -> Vec3 {
        match self.march_until_hit(origin, direction) {
            Some(hit_point) => self.color_on_surface(hit_point, remaining_bounces),
            None => self.sky_color,
        }
    }
}
