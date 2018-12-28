mod vector;

fn main() {
    let vec = vector::Vector3 {
        x: 4.0,
        y: 5.0,
        z: 2.0,
    };

    let norm = vec.normalized ();

    println!("Vector {:?}", vec);
    println!("Normalized {:?}", norm);
}