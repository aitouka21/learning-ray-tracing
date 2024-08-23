use rand::Rng;

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

/// point3 is just an alias for vec3, but useful for geometric clarity in the code.
pub type Point3 = Vec3;

impl Vec3 {
    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn at(&self, idx: usize) -> f64 {
        self.e[idx]
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_squared())
    }

    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let v = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );

            if v.len_squared() < 1.0 {
                break v;
            }
        }
        .unit()
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Self {
        v - 2.0 * Self::dot(v, n) * n
    }

    pub fn len_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.e[0]) < s) && (f64::abs(self.e[1]) < s) && (f64::abs(self.e[2]) < s)
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
        a.e[0] * b.e[0] + a.e[1] * b.e[1] + a.e[2] * b.e[2]
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3::new(
            a.e[1] * b.e[2] - a.e[2] * b.e[1],
            a.e[2] * b.e[0] - a.e[0] * b.e[2],
            a.e[0] * b.e[1] - a.e[1] * b.e[0],
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.e[0], self.e[1], self.e[2])
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.e[i] += rhs.e[i];
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}
impl std::ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl std::ops::Mul<&f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: &f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self)
    }
}

impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.e[i] *= rhs;
        }
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
