fn main() {
    let vec = Vector3{
        x: 4.0,
        y: 5.0,
        z: 2.0,
    };

    let norm = vec.normalized ();

    println!("{:?}", vec);
    println!("{:?}", norm);
}

#[derive(Debug)]
struct Vector3  {
    x : f32,
    y : f32,
    z : f32,
}

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

    fn magnitude (&self) -> f32 {
        let sqr_mag = self.x * self.x + self.y * self.y + self.z + self.z;
        sqr_mag.sqrt()
    }
}