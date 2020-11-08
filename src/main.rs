use image::{ImageBuffer, RgbImage};
use rand::{rngs::ThreadRng, Rng};
use rand_distr::{Distribution, Normal};

#[derive(Clone)]
struct Coord3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Coord3D {
    fn zero() -> Coord3D {
        Coord3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    const fn new(x: f32, y: f32, z: f32) -> Coord3D {
        Coord3D { x: x, y: y, z: z }
    }

    fn distance_to(&self, other: &Coord3D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn normalize(&mut self) {
        let length = self.distance_to(&Coord3D::zero());
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    fn add(&self, other: &Coord3D) -> Coord3D {
        Coord3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    fn sub(&self, other: &Coord3D) -> Coord3D {
        Coord3D::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    fn mul(&self, other: &Coord3D) -> Coord3D {
        Coord3D::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    fn abs(&self) -> Coord3D {
        Coord3D::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    fn scale(&self, factor: f32) -> Coord3D {
        Coord3D::new(self.x * factor, self.y * factor, self.z * factor)
    }

    fn dot_product(&self, other: &Coord3D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

struct Ray {
    origin: Coord3D,
    direction: Coord3D,
}

struct Sphere {
    origin: Coord3D,
    radius: f32,
}

impl Sphere {
    fn distance_to(&self, point: &Coord3D) -> f32 {
        self.origin.distance_to(point) - self.radius
    }
}

struct Cube {
    origin: Coord3D,
    size: Coord3D,
}

impl Cube {
    fn distance_to(&self, point: &Coord3D) -> f32 {
        let distance = point.sub(&self.origin).abs();
        let distance = distance.sub(&self.size);
        distance.x.max(distance.y.max(distance.z))
    }
}

fn distance_field(point: &Coord3D) -> f32 {
    let sphere = Sphere {
        origin: Coord3D::new(0.0, 0.0, 10.0),
        radius: 1.0,
    };
    let sphere2 = Sphere {
        origin: Coord3D::new(1.0, 0.0, 10.0),
        radius: 0.5,
    };
    let floor = Cube {
        origin: Coord3D::new(0.0, 1.0, 10.0),
        size: Coord3D::new(5.0, 0.1, 5.0),
    };
    let mut min = sphere
        .distance_to(point)
        .min(sphere2.distance_to(point))
        .min(floor.distance_to(point));
    for n in 0..11 {
        let dist = Sphere {
            origin: Coord3D::new((n as f32) * 0.5 - 2.5, 0.9, 9.0),
            radius: 0.3,
        }
        .distance_to(point);
        min = min.min(dist);
    }
    min
}

fn normal_at(point: &Coord3D) -> Coord3D {
    const EPSILON: f32 = 1e-4;
    let df_center = distance_field(point);
    let df_x = distance_field(&point.add(&Coord3D::new(EPSILON, 0.0, 0.0)));
    let df_y = distance_field(&point.add(&Coord3D::new(0.0, EPSILON, 0.0)));
    let df_z = distance_field(&point.add(&Coord3D::new(0.0, 0.0, EPSILON)));
    let mut normal = Coord3D::new(df_x - df_center, df_y - df_center, df_z - df_center);
    normal.normalize();
    normal
}

fn color_at(point: &Coord3D) -> Coord3D {
    let mut sun_dir = Coord3D::new(1.0, 1.0, 0.5);
    sun_dir.normalize();
    let normal = normal_at(point);
    let mut brightness = -sun_dir.dot_product(&normal);
    if brightness < 0.0 {
        brightness = 0.0;
    }
    let ray_dir = sun_dir.scale(-1.0);
    let ray_origin = point.add(&normal.scale(MIN * 2.0));
    let hit_point = shoot_ray(ray_origin, ray_dir);
    if distance_field(&hit_point) <= MIN {
        brightness = 0.0;
    }
    Coord3D::new(1.0, 0.0, 1.0).scale(brightness)
}

fn random_bounce_direction(normal: &Coord3D) -> Coord3D {
    let mut rng = rand::thread_rng();
    let random_point_on_sphere = loop {
        let random_vec = Coord3D::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        // Throw away all vectors with length greater than one.
        if random_vec.distance_to(&Coord3D::zero()) > 1.0 {
            continue;
        }
        let mut result = random_vec;
        result.normalize();
        break result;
    };
    let mut result = random_point_on_sphere.add(normal);
    result.normalize();
    result
}

fn realistic_color_at(point: &Coord3D, extra_bounces: u32) -> Coord3D {
    // Start with no light being received.
    let mut color = Coord3D::zero();
    let normal = normal_at(point);
    let ray_origin = point.add(&normal.scale(MIN * 2.0));
    let surface_color = Coord3D::new(1.0, 0.9, 0.7);
    // Check if we get sunlight.
    let mut sun_dir = Coord3D::new(1.0, 1.0, 0.5);
    sun_dir.normalize();
    let sun_ray_dir = sun_dir.scale(-1.0);
    let sun_hit = shoot_ray(ray_origin.clone(), sun_ray_dir);
    if distance_field(&sun_hit) >= MAX {
        // We reached the sky, so nothing is causing us to shadow the sun.
        let brightness = -normal.dot_product(&sun_dir);
        if brightness > 0.0 {
            color = color.add(&surface_color.scale(brightness));
        }
    }
    // Do bounce lighting
    if extra_bounces > 0 {
        let bounce_ray_dir = random_bounce_direction(&normal);
        let brightness = normal.dot_product(&bounce_ray_dir);
        let bounce_hit = shoot_ray(ray_origin, bounce_ray_dir);
        let light = if distance_field(&bounce_hit) >= MAX {
            Coord3D::new(0.4, 0.7, 1.0)
        } else {
            realistic_color_at(&bounce_hit, extra_bounces - 1)
        };
        if brightness > 0.0 {
            color = color.add(&light.mul(&surface_color).scale(brightness));
        }
    }
    color
}

fn shoot_ray(origin: Coord3D, direction: Coord3D) -> Coord3D {
    let mut ray = Ray {
        origin: origin,
        direction: direction,
    };
    loop {
        let distance = distance_field(&ray.origin);
        if distance >= MAX || distance <= MIN {
            break ray.origin;
        }
        ray.origin = ray.origin.add(&ray.direction.scale(distance));
    }
}

const X: u32 = 200;
const Y: u32 = 200;

const MAX: f32 = 1e5;
const MIN: f32 = 1e-5;
const CAMERA_SIZE: f32 = 0.3;
const MAX_BOUNCES: u32 = 5;
const EXPOSURE: f32 = 0.8;
const NUM_SAMPLES: u32 = 128;

fn sample(x: u32, y: u32) -> Coord3D {
    let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 0.2).unwrap();
    let dx = dist.sample(&mut rng);
    let dy = dist.sample(&mut rng);
    let ray_dir_x = (((x as f32 + dx) / X as f32) - 0.5) * 2.0 * CAMERA_SIZE;
    let ray_dir_y = (((y as f32 + dy) / Y as f32) - 0.5) * 2.0 * CAMERA_SIZE;
    let ray_dir_z = 1.0;
    let mut ray_dir = Coord3D::new(ray_dir_x, ray_dir_y, ray_dir_z);
    ray_dir.normalize();
    let result = shoot_ray(Coord3D::zero(), ray_dir);
    let hit_sphere = distance_field(&result) <= MIN;
    if hit_sphere {
        realistic_color_at(&result, MAX_BOUNCES)
    } else {
        Coord3D::new(0.0, 0.0, 1.0)
    }
}

fn main() {
    let mut buf: RgbImage = ImageBuffer::new(X, Y);
    for x in 0..X {
        println!("{}", x);
        for y in 0..Y {
            let mut color = Coord3D::zero();
            for _ in 0..NUM_SAMPLES {
                color = color.add(&sample(x, y));
            }
            color = color.scale(EXPOSURE / NUM_SAMPLES as f32);
            let color = [
                (color.x * 255.0) as u8,
                (color.y * 255.0) as u8,
                (color.z * 255.0) as u8,
            ];
            buf.put_pixel(x, y, color.into());
        }
    }
    buf.save("test.png").unwrap();
}
