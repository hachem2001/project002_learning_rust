use std::ops::{Add, Sub};
#[derive(Debug, Copy, Clone)]
struct Vector(f32, f32);

struct Triangle(Vector, Vector, Vector);


impl Vector {
    fn length(&self) -> f32 {
        f32::powf(self.0*self.0 + self.1*self.1, 0.5)
    }
}
impl Add for Vector {
    type Output = Self;
    fn add(self, other:Self) -> Self {
        Self(self.0+other.0, self.1+other.1)
    }
}
impl Sub for Vector {
    type Output = Self;
    fn sub(self, other:Self) -> Self {
        Self(self.0-other.0, self.1-other.1)
    }
}


impl Triangle {
    fn surface(&self) -> f32 {
        // I'll be using heron's formula - there are more efficient formulas using coordinates though.
        let (vab, vbc, vca) = (self.1 - self.0, self.2 - self.1, self.0 - self.2); // three vectors for three vertices
        let (l1, l2, l3) : (f32, f32, f32) = (vab.length(), vbc.length(), vca.length()); // three lengths
        let s = (l1 + l2 + l3)/2.; // the famous constant of Heron's formula
        f32::powf(s*(s-l1)*(s-l2)*(s-l3), 0.5) // return the result
    }
}


fn main() {
    println!("Hello, world!");
    let t = Triangle( Vector(0., 0.), Vector(1., 0.), Vector(0., 1.));

    println!("The surface of the triangle is {}", t.surface());
}