use std::ops::{Add, Sub}

#[derive(Clone)]
struct Vector(f32, f32);
struct Triangle(Vector, Vector, Vector);

impl Vector {
    fn length(&self) -> f32 {
        f32::powf(self.0*self.0 + self.1*self.1, 0.5)
    }
}

impl Add for Vector {
    
}

impl Triangle {
    fn surface(&self) -> f32 {
        // I'll be using heron's formula - there are more efficient formulas using coordinates though.
        let (l1, l2, l3) : (f32, f32, f32) = (self.0.length(), self.0.length(), self.0.length());
        let s = (l1 + l2 + l3)/2.;
        f32::powf(s*(s-l1)*(s-l2)*(s-l3), 0.5)
    }
}


fn main() {
    println!("Hello, world!");
    let t = Triangle( Vector(0., 0.), Vector(1., 0.), Vector(0., 1.));

    println!("The surface of the triangle is {}", t.surface());
}