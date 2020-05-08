use std::cmp::PartialEq;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    x: f64,
    y: f64,
}
// Implement Vec2 functions
impl Vec2 {
    pub fn new(x: f64, y: f64)  -> Vec2 {
        Vec2 {
            x,
            y
        }
    }

    pub fn magnitude (&self) -> f64 {
        return f64::sqrt(self.x*self.x + self.y*self.y);
    }

    pub fn normalise (&self) -> Vec2 {
        Vec2 {
            x: self.x/self.magnitude(),
            y: self.y/self.magnitude()
        }
    }

    pub fn multiply (&self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x*scalar,
            y: self.y*scalar
        }
    }

    pub fn grid (&self) -> Vec2 {
        Vec2{
            x: f64::round(self.x),
            y: f64::round(self.y)
        }
    }
}

impl PartialEq for Vec2 {
    fn eq (&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x+other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
