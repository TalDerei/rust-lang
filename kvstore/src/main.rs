use std::*;
use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is '{}' and the value is '{}", key, value);

    let mut database = Database::new().unwrap();
    database.insert(key.clone(), value.clone());
    database.insert(key, value);
    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, io::Error> {
        // The '?' operator is equivalent to the code below
        // let contents = match fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => { return Err(error) }
        // };

        // Instantiate HashMap object in-memory
        let mut map = HashMap::new();

        // Read and parse 'kv.db' file, populate HashMap
        let contents = fs::read_to_string("kv.db")?; 
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let k = chunks.next().expect("No key!");
            let v = chunks.next().expect("No value!");
            map.insert(k.to_owned(), v.to_owned());
        }

        Ok(Database { map, flush: false })
    }

    // Insert method denoted by '&mut self'
    fn insert(&mut self, k: String, v: String) {
        match self.map.insert(k, v) {
            None => println!("None!"),
            Some(value) => println!("Some! {}", value),
        };
    }

    fn flush(mut self) -> std::io::Result<()> {
        // API designed to take ownership of self on invocation
        self.flush = true;
        do_flush(&self)
    }
}

// Drop trait triggered when Database exits scope
impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    // &self.map means for loop does not take ownership of self
    for (key, value) in &database.map { 
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    fs::write("kv.db", contents)
}