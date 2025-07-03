use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use num_traits::Float;

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub e: [T; 3],
}

impl<T> Vec3<T> 
where 
    T: Copy
{
    pub fn new(e0: T, e1: T, e2: T) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> T {
        self.e[0]
    }
    pub fn y(&self) -> T {
        self.e[1]
    }
    pub fn z(&self) -> T {
        self.e[2]
    }
}

impl<T> Vec3<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>
{
    pub fn length_squared(&self) -> T {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl<T> Vec3<T>
where
    T: Float
{
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Copy
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign + Copy
{
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Copy
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: SubAssign + Copy
{
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T> + Copy
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy
{
    type Output = Self;

    fn mul(self, t: T) -> Self::Output {
        Vec3::new(
            self.e[0] * t,
            self.e[1] * t,
            self.e[2] * t,
        )
    }
}

impl<T> MulAssign<T> for Vec3<T>
where
    T: MulAssign + Copy
{
    fn mul_assign(&mut self, t: T) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy
{
    type Output = Self;

    fn div(self, t: T) -> Self::Output {
        Vec3::new(
            self.e[0] / t,
            self.e[1] / t,
            self.e[2] / t,
        )
    }
}

impl<T> DivAssign<T> for Vec3<T>
where
    T: Float + MulAssign
{
    fn div_assign(&mut self, t: T) {
        self.mul_assign(T::one() / t);
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T> + Copy
{
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, v: Vec3<f64>) -> Self::Output {
        v * self
    }
}

pub fn dot<T>(u: &Vec3<T>, v: &Vec3<T>) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Copy
{
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross<T>(u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy
{
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector<T>(v: Vec3<T>) -> Vec3<T>
where
    T: Float
{
    v / v.length()
}
pub type Point3 = Vec3<f64>;
