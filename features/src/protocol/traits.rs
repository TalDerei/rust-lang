// Traits are similair to interfaces, defining a set of common behaviors / methods 
// for different types. To implement a trait for a particular type, you use the `impl` keyword

// &'static str is a string slice that's stored in static memory
// for the duration of the program's runtime (i.e. static lifetime)
pub struct Sheep { naked: bool, name: &'static str }

pub trait Animal {
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    fn origin() -> Sheep {
        Sheep { naked: true, name: "bob" }
    }

    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
    // &self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Sheep`
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    
    // `&mut self` is a mutable reference Self
    pub fn sheer(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        }
        else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}