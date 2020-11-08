use num_traits::NumCast;
use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns a random vector with mangnitude 1, with the random distribution guaranteeing that
    /// all directions are equally likely, I.E. the vectors are all equally distributed along the
    /// surface of the unit sphere.
    pub fn random_unit_vec() -> Self {
        let mut rng = rand::thread_rng();
        loop {
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
        }
    }

    pub fn dot<T: Into<Self>>(self, other: T) -> f32 {
        let other = other.into();
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross<T: Into<Self>>(self, other: T) -> Self {
        let other = other.into();
        Self {
            x: self.y * other.z,
            y: self.z * other.x,
            z: self.x * other.y,
        }
    }

    /// Returns two vectors such that each vector is perpendicular to both each other and the
    /// original vector.
    pub fn make_two_perpendicular(self) -> (Self, Self) {
        let v1 = if self.x == 0.0 && self.y == 0.0 {
            (0, 1, 0).into()
        } else {
            (-self.y, self.x, 0).into()
        };
        let v2 = self.cross(v1);
        (v1, v2)
    }

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn distance<T: Into<Self>>(self, other: T) -> f32 {
        (self - other.into()).magnitude()
    }

    pub fn normalized(self) -> Self {
        self / self.magnitude()
    }

    pub fn abs(self) -> Self {
        (self.x.abs(), self.y.abs(), self.z.abs()).into()
    }
}

impl<T: NumCast + Copy> From<[T; 3]> for Vec3 {
    fn from(other: [T; 3]) -> Self {
        Self::new(
            num_traits::cast(other[0]).unwrap(),
            num_traits::cast(other[1]).unwrap(),
            num_traits::cast(other[2]).unwrap(),
        )
    }
}

impl<T: NumCast + Copy> From<[T; 2]> for Vec3 {
    fn from(other: [T; 2]) -> Self {
        (other[0], other[1]).into()
    }
}

impl<T: NumCast + Copy, U: NumCast + Copy, V: NumCast + Copy> From<(T, U, V)> for Vec3 {
    fn from((x, y, z): (T, U, V)) -> Self {
        Self::new(
            num_traits::cast(x).unwrap(),
            num_traits::cast(y).unwrap(),
            num_traits::cast(z).unwrap(),
        )
    }
}

impl<T: NumCast + Copy, U: NumCast + Copy> From<(T, U)> for Vec3 {
    fn from((x, y): (T, U)) -> Self {
        (x, y, 0).into()
    }
}

impl From<f32> for Vec3 {
    fn from(other: f32) -> Self {
        (other, other, other).into()
    }
}

impl From<f64> for Vec3 {
    fn from(other: f64) -> Self {
        (other, other, other).into()
    }
}

impl From<i32> for Vec3 {
    fn from(other: i32) -> Self {
        (other, other, other).into()
    }
}

impl From<u32> for Vec3 {
    fn from(other: u32) -> Self {
        (other, other, other).into()
    }
}

impl From<i64> for Vec3 {
    fn from(other: i64) -> Self {
        (other, other, other).into()
    }
}

impl From<u64> for Vec3 {
    fn from(other: u64) -> Self {
        (other, other, other).into()
    }
}

impl From<isize> for Vec3 {
    fn from(other: isize) -> Self {
        (other, other, other).into()
    }
}

impl From<usize> for Vec3 {
    fn from(other: usize) -> Self {
        (other, other, other).into()
    }
}

impl<T: Into<Vec3>> Add<T> for Vec3 {
    type Output = Vec3;

    fn add(self, other: T) -> Vec3 {
        let other = other.into();
        (self.x + other.x, self.y + other.y, self.z + other.z).into()
    }
}

impl<T: Into<Vec3>> AddAssign<T> for Vec3 {
    fn add_assign(&mut self, other: T) {
        let other = other.into();
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: Into<Vec3>> Sub<T> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: T) -> Vec3 {
        let other = other.into();
        (self.x - other.x, self.y - other.y, self.z - other.z).into()
    }
}

impl<T: Into<Vec3>> SubAssign<T> for Vec3 {
    fn sub_assign(&mut self, other: T) {
        let other = other.into();
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: Into<Vec3>> Mul<T> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: T) -> Vec3 {
        let other = other.into();
        (self.x * other.x, self.y * other.y, self.z * other.z).into()
    }
}

impl<T: Into<Vec3>> MulAssign<T> for Vec3 {
    fn mul_assign(&mut self, other: T) {
        let other = other.into();
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T: Into<Vec3>> Div<T> for Vec3 {
    type Output = Vec3;

    fn div(self, other: T) -> Vec3 {
        let other = other.into();
        (self.x / other.x, self.y / other.y, self.z / other.z).into()
    }
}

impl<T: Into<Vec3>> DivAssign<T> for Vec3 {
    fn div_assign(&mut self, other: T) {
        let other = other.into();
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<T: Into<Vec3>> Rem<T> for Vec3 {
    type Output = Vec3;

    fn rem(self, other: T) -> Vec3 {
        let other = other.into();
        (self.x % other.x, self.y % other.y, self.z % other.z).into()
    }
}

impl<T: Into<Vec3>> RemAssign<T> for Vec3 {
    fn rem_assign(&mut self, other: T) {
        let other = other.into();
        self.x %= other.x;
        self.y %= other.y;
        self.z %= other.z;
    }
}
