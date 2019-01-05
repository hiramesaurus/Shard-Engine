#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector3  {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vector3 {
    pub fn cross (&self, other: &Vector3) -> Vector3 {
        return Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.y * other.y - self.y * other.x
        }
    }

    pub fn dot (&self, other: &Vector3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn normalized (&self) -> Vector3 {
        let mag = self.magnitude();
        return Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn magnitude (&self) -> f32 {
        let sqr_mag = self.x * self.x + self.y * self.y + self.z + self.z;
        return sqr_mag.sqrt()
    }
}