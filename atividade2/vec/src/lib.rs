use std::{ops, vec};

pub struct Vec3 {
    pub coords: Vec<f64>,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            coords: vec![x, y, z],
        }
    }

    fn sum(mut self, other: &Vec3) -> Self {
        for i in 0..3 {
            self.coords[i] += other.coords[i];
        }
        self
    }

    fn multiply(mut self, factor: f64) -> Self {
        for i in 0..3 {
            self.coords[i] *= factor;
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
        self.coords[0]*self.coords[0] + self.coords[1]*self.coords[1] + self.coords[2]*self.coords[2]
    }
}

pub fn v3_to_string(u: &Vec3) -> String {
    u.coords.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" ")
}

impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { coords: vec![self.coords[0]+rhs.coords[0],self.coords[1]+rhs.coords[1],self.coords[2]+rhs.coords[2]] }
    }
}

impl ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { coords: vec![self.coords[0]-rhs.coords[0],self.coords[1]-rhs.coords[1],self.coords[2]-rhs.coords[2]] }
    }
}

impl ops::Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { coords: vec![self.coords[0]*rhs.coords[0],self.coords[1]*rhs.coords[1],self.coords[2]*rhs.coords[2]] }
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 { coords: vec![self*rhs.coords[0],self*rhs.coords[1],self*rhs.coords[2]] }
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
    u.coords[0]*v.coords[0]+u.coords[1]*v.coords[1]+u.coords[2]*v.coords[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 { coords: vec![
            u.coords[1]*v.coords[2]-u.coords[2]*v.coords[1],
            u.coords[2]*v.coords[0]-u.coords[0]*v.coords[2],
            u.coords[0]*v.coords[1]-u.coords[1]*v.coords[0]] }
}

pub fn unit_vector(u: &Vec3) -> Vec3 {
    u/u.length()
}

pub fn write_color(pixel_color: Vec3) {
    println!("{} {} {}", (255.999*pixel_color.coords[0]) as u32, (255.999*pixel_color.coords[1]) as u32, (255.999*pixel_color.coords[2]) as u32)
}

/// TESTS


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
