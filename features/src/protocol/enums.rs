// Enum is a type that may take on one of many different possible variants

pub enum Event {
    // Variants
    PageLoad, 
    PageUnload, 
    KeyPress(char), // ~ tuple struct
    Paste(String),
    Click { x: i64, y: i64}, // ~ c-like struct
}

pub fn inspect(event: Event) {
    match event {
        Event::PageLoad => println!("page loaded"),
        Event::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        Event::KeyPress(c) => println!("pressed '{}'.", c),
        Event::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        Event::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}