use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

pub type Color = Vec3;

#[allow(unused)]
pub static VEC3_UNIT: Vec3 = Vec3(1.0, 1.0, 1.0);
#[allow(unused)]
pub static VEC3_ZERO: Vec3 = Vec3(0.0, 0.0, 0.0);
#[allow(unused)]
pub static VEC3_UP: Vec3 = Vec3(0.0, 1.0, 0.0);
#[allow(unused)]
pub static VEC3_DOWN: Vec3 = Vec3(0.0, -1.0, 0.0);
#[allow(unused)]
pub static VEC3_LEFT: Vec3 = Vec3(-1.0, 0.0, 0.0);
#[allow(unused)]
pub static VEC3_RIGHT: Vec3 = Vec3(1.0, 0.0, 0.0);
#[allow(unused)]
pub static VEC3_FORWARD: Vec3 = Vec3(0.0, 0.0, 1.0);
#[allow(unused)]
pub static VEC3_BACKWARD: Vec3 = Vec3(0.0, 0.0, -1.0);

pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
    lhs.dot(rhs)
}

impl Default for Vec3 {
    fn default() -> Self {
        VEC3_ZERO
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn r(&self) -> f32 {
        self.0
    }

    pub fn g(&self) -> f32 {
        self.1
    }

    pub fn b(&self) -> f32 {
        self.2
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            -(self.0 * rhs.2 - self.2 * rhs.0),
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalize(&self) -> Vec3 {
        self.div(self.length())
    }

    pub fn normalize_into(self) -> Vec3 {
        let len = self.length();
        self / len
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn test_ownership() {
        let a = Vec3::new(10.0, 10.0, 10.0);
        let b = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a - b, Vec3::new(8.0, 8.0, 8.0));
        assert_eq!(a + b, Vec3::new(12.0, 12.0, 12.0));
    }
}
