mod vector;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

fn main() {
    let vec1 = vector::Vector3 {
        x: 4.0,
        y: -5.0,
        z: 2.0,
    };

    let vec2 = vector::Vector3 {
        x: 2.0,
        y: 1.0,
        z: 0.0,
    };

    let norm1 = vec1.normalized();
    let norm2 = vec2.normalized();
    let cross = norm1.cross(&norm2);
    let dot = norm1.dot(&norm2);

    println!("Vector 1: {:?} | Normalized: {:?}", vec1, norm1);
    println!("Vector 2: {:?} | Normalized: {:?}", vec2, norm2);


    println! ("Cross {:?}", cross);
    println!("Dot {:?}", dot);

    let serialized = serde_json::to_string(&norm1).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: vector::Vector3 = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

}