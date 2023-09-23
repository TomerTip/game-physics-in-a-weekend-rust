mod vec {
    pub mod vec2d;
    pub mod vec3d;
}
use vec::vec2d::Vec2d;
use vec::vec3d::Vec3d;


fn main() {
    let vec = Vec2d::new(1.0, 1.0);
    let vec2 = Vec2d::new(20.0, 20.0);

    println!("Vector 1: {:?}", vec);
    println!("mag: {:?}", vec.get_magnitude());
    println!("normal: {:?}", vec.normalize());
    println!("add: {:?}", vec + vec);
    println!("sub: {:?}", vec2 - vec);
    println!("mul: {:?}", vec * vec);
    println!("div: {:?}", vec / 2.0);
    println!("equal: {:?}", vec == vec);
    println!("not equal: {:?}", vec != vec);

    let vec3 = Vec3d::new(1.0, 1.0, 2.0);
    let vec4 = Vec3d::new(20.0, 20.0, 10.0);

    println!("Vector 1: {:?}", vec3);
    println!("mag: {:?}", vec3.get_magnitude());
    println!("normal: {:?}", vec3.normalize());
    println!("add: {:?}", vec3 + vec4);
    println!("sub: {:?}", vec4 - vec3);
    println!("mul: {:?}", vec3 * vec4);
    println!("div: {:?}", vec3 / 2.0);
    println!("equal: {:?}", vec3 == vec3);
    println!("not equal: {:?}", vec3 != vec4);


}
