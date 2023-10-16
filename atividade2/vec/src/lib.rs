use std::{ops, vec};

pub struct Vec3 {
    pub values: Vec<f64>,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            values: vec![x, y, z],
        }
    }

    fn sum(mut self, other: &Vec3) -> Self {
        for i in 0..3 {
            self.values[i] += other.values[i];
        }
        self
    }

    fn multiply(mut self, factor: f64) -> Self {
        for i in 0..3 {
            self.values[i] *= factor;
        }
        self
    }

    fn divide(self, factor: f64) -> Self {
        self.multiply(1.0/factor)
    }

    fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    fn length_squared(&self) -> f64 {
        self.values[0]*self.values[0] + self.values[1]*self.values[1] + self.values[2]*self.values[2]
    }
}

pub fn v3_to_string(u: &Vec3) -> String {
    u.values.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ")
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 { values: vec![self.values[0]+rhs.values[0],self.values[1]+rhs.values[1],self.values[2]+rhs.values[2]] }
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 { values: vec![self.values[0]-rhs.values[0],self.values[1]-rhs.values[1],self.values[2]-rhs.values[2]] }
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 { values: vec![self.values[0]*rhs.values[0],self.values[1]*rhs.values[1],self.values[2]*rhs.values[2]] }
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 { values: vec![self*rhs.values[0],self*rhs.values[1],self*rhs.values[2]] }
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        rhs*self
    }
}


impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0/rhs)*self
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.values[0]*v.values[0]+u.values[1]*v.values[1]+u.values[2]*v.values[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 { values: vec![
            u.values[1]*v.values[2]-u.values[2]*v.values[1],
            u.values[2]*v.values[0]-u.values[0]*v.values[2],
            u.values[0]*v.values[1]-u.values[1]*v.values[0]] }
}

pub fn unit_vector(u: &Vec3) -> Vec3 {
    u/u.length()
}

pub fn write_color(pixel_color: Vec3) {
    println!("{} {} {}", (255.999*pixel_color.values[0]) as u32, (255.999*pixel_color.values[1]) as u32, (255.999*pixel_color.values[2]) as u32)
}

/// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_test() {
        let x: Vec3 = Vec3::new(2.0, 3.0, 4.0);
        let other: Vec3 = Vec3::new(2.0, 3.0, 4.0);
        let result = x.sum(&other);
        assert_eq!(result.values[0], 4.0);
        assert_eq!(result.values[1], 6.0);
        assert_eq!(result.values[2], 8.0);
    }

    #[test]
    fn multiply_test() {
        let x: Vec3 = Vec3::new(2.0, 3.0, 4.0);
        let other = 3.0;
        let result = x.multiply(other);
        assert_eq!(result.values[0], 6.0);
        assert_eq!(result.values[1], 9.0);
        assert_eq!(result.values[2], 12.0);
    }

    #[test]
    fn div_test() {
        let x: Vec3 = Vec3::new(2.0, 3.0, 4.0);
        let other = 2.0;
        let result = x.divide(other);
        assert_eq!(result.values[0], 1.0);
        assert_eq!(result.values[1], 1.5);
        assert_eq!(result.values[2], 2.0);
    }

    #[test]
    fn length_test() {
        let x = Vec3::new(2.0,3.0,4.0);
        assert_eq!(5.38516480713450403125071049154032955629512016164478883768038867,x.length())
    }

    #[test]
    fn inline_sum_test() {
        let x = Vec3::new(2.0,3.0,4.0);
        let other: Vec3 = Vec3::new(3.0,5.0,8.0);
        assert_eq!(5.0,(&x+&other).values[0]);
        assert_eq!(8.0,(&x+&other).values[1]);
        assert_eq!(12.0,(&x+&other).values[2]);
    }

    #[test]
    fn inline_sub_test() {
        let x = Vec3::new(2.0,3.0,4.0);
        let other: Vec3 = Vec3::new(3.0,5.0,8.0);
        assert_eq!(-1.0,(&x-&other).values[0]);
        assert_eq!(-2.0,(&x-&other).values[1]);
        assert_eq!(-4.0,(&x-&other).values[2]);
    }
    #[test]
    fn inline_mul_vec_test() {
        let x = Vec3::new(2.0,3.0,4.0);
        let other: Vec3 = Vec3::new(3.0,5.0,8.0);
        assert_eq!(6.0,(&x*&other).values[0]);
        assert_eq!(15.0,(&x*&other).values[1]);
        assert_eq!(32.0,(&x*&other).values[2]);
    }

    #[test]
    fn inline_mul_factor_test() {
        let x = 10.0;
        let other: Vec3 = Vec3::new(3.0,5.0,8.0);
        assert_eq!(30.0,(x*&other).values[0]);
        assert_eq!(50.0,(x*&other).values[1]);
        assert_eq!(80.0,(x*&other).values[2]);
    }

    #[test]
    fn inline_mul_factor_inverted_test() {
        let x = 10.0;
        let other: Vec3 = Vec3::new(3.0,5.0,8.0);
        assert_eq!(30.0,(&other*x).values[0]);
        assert_eq!(50.0,(&other*x).values[1]);
        assert_eq!(80.0,(&other*x).values[2]);
    }

    #[test]
    fn inline_div_factor_test() {
        let x = Vec3::new(23.4,85.2,90.1);
        let other: f64 = 2.5;
        assert_eq!(9.36,(&x/other).values[0]);
        assert_eq!(34.080000000000005,(&x/other).values[1]);
        assert_eq!(36.04,(&x/other).values[2]);
    }

    #[test]
    fn dot_test() {
        let x = Vec3::new(23.4,85.2,90.1);
        let other: Vec3 = Vec3::new(3.3, 2.1, 3.9);
        let result = dot(&x,&other);
        assert_eq!(607.53,result);
    }

    #[test]
    fn cross_test() {
        let x = Vec3::new(23.4,85.2,90.1);
        let other: Vec3 = Vec3::new(3.3, 2.1, 3.9);
        let result = cross(&x,&other);
        assert_eq!(143.07000000000002,result.values[0]);
        assert_eq!(206.07,result.values[1]);
        assert_eq!(-232.01999999999998,result.values[2]);
    }
}
