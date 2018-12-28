#[derive(Debug)]
pub struct Vector3  {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vector3 {
    pub fn cross (&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn normalized (&self) -> Vector3 {
        let mag = self.magnitude();
        Vector3{
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn magnitude (&self) -> f32 {
        let sqr_mag = self.x * self.x + self.y * self.y + self.z + self.z;
        sqr_mag.sqrt()
    }
}