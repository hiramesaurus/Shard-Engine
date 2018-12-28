#[derive(debug)]
struct Vector3  {
    x : f32,
    y : f32,
    z : f32,
}

use std::num;

impl Vector3 {
    fn cross (&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    fn normalized (&self){
        let mag = self.magnitude();
        Vector3{
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        };
    }

    fn magnitude (&self) -> f64 {
        let sqr_mag = self.x * self.x + self.y * self.y + self.z + self.z;
        num::sqrt (sqr_mag)
    }
}