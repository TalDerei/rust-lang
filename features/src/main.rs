mod protocol;
use protocol::{traits::*, structs::*, enums::*, generics::*};

fn main() {
    traits_example();
    struct_example();
    enum_example();
    generics_example();
}

fn traits_example() {    
    let mut dolly: Sheep = Animal::new("dolly");
    dolly.talk();
    dolly.sheer();   
}

fn struct_example() {
    // Instantiate Point struct
    let point: Point = Point {x: 5.0, y: 10.0};
    println!("point coordinates: ({}, {})", point.x, point.y);     
    
    // Instantiate tuple struct
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
}

fn enum_example() {
    let pressed = Event::KeyPress('x');
    let pasted  = Event::Paste("my text".to_owned());
    let click   = Event::Click { x: 20, y: 80 };
    let load    = Event::PageLoad;
    let unload  = Event::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

fn generics_example() {
    // `Single` is concrete and explicitly takes `A`
    let _s = Single(A);

    // `SingleGen` can be explicitly or implicitely defined
    let _explicit: SingleGen<char> = SingleGen('a');
    let _implicit = SingleGen(A);
}