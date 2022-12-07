// Attribute to hide warning for unused code
#![allow(dead_code)]

// Implements the `std::fmt::Debug` trait for struct or enum,
// enabling the use of the {:?} formatting syntax
#[derive(Debug)]
pub struct Person {
    pub name: String, 
    pub age: u8,
}

// Unit structs are field-less and useful for generics
pub struct Unit;

// Tuple struct
pub struct Pair(pub i32, pub f32);

// Struct with two fields
pub struct Point {
    pub x: f32, 
    pub y: f32,
}

// Struct inside of another struct
pub struct Rectangle {  
    pub top_left: Point, 
    pub top_right: Point, 
}

