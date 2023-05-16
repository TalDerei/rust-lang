mod protocol;
use protocol::{structs::*, generics::*};

fn main() {
    struct_example();
    generics_example();
}

fn struct_example() {
    // Instantiate Point struct
    let point: Point = Point {x: 5.0, y: 10.0};
    println!("point coordinates: ({}, {})", point.x, point.y);     
    
    // Instantiate tuple struct
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
}

fn generics_example() {
    // `Single` is concrete and explicitly takes `A`
    let _s = Single(A);

    // `SingleGen` can be explicitly or implicitely defined
    let _explicit: SingleGen<char> = SingleGen('a');
    let _implicit = SingleGen(A);
}