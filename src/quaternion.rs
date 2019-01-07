pub struct Quaternion {
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}

impl Quaternion {

    pub fn multiply (&self, other: &Quaternion) -> Quaternion {
        Quaternion{
            x: self.x * other.x - self.y * other.y - self.z * other.x - self.w * other.w,
            y: self.x * other.y + self.y * other.x + self.z * other.w - self.w * other.z,
            z: self.x * other.z - self.y * other.w + self.z * other.x + self.w * other.y,
            w: self.x * other.w + self.y * other.z - self.z * other.y + self.w * other.x,
        }
    }
}