// The guard pattern is a design pattern to restrict access to a value of a certain type.
// The value can only be accessed through a guard, which is essentially a wrapper struct.
// It usually implements the DropThis links to official Rust documentation trait to do
// something when the guard is destroyed. E.g. Mutex

use std::cell::RefCell;
use std::ops::Deref;

struct Pool<T> {
    items: RefCell<Vec<T>>,
}

impl<T: PoolItem> Pool<T> {
    fn new() -> Self {
        Self { items: RefCell::new(Vec::new()) }
    }

    // We have a non-exclusive, immutable reference to self. We don't want 
    // exclusive access to the pool, so use RefCell for interior mutability
    fn get(&self) -> PoolGaurd<T> {
        let item = match self.items.borrow_mut().pop() {
            Some(item) => { item },
            None => T::new(),
        };
        PoolGaurd { 
            inner: Some(item), 
            items: &self.items
        }
    }
}

trait PoolItem {
    fn new() -> Self;
}

// PoolGaurd will return the resources back to the pool upon drop
struct PoolGaurd<'a, T> {
    inner: Option<T>,
    items: &'a RefCell<Vec<T>>
}

impl<'a, T> Drop for PoolGaurd<'a, T> {
    fn drop(&mut self) {
        let item = self.inner.take().unwrap();
        // Return the item to the pool
        self.items.borrow_mut().push(item);
    }
}

impl <'a, T> Deref for PoolGaurd<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        struct Awesome;
        impl Awesome {
            fn do_thing(&self) {
                println!("TEST!");
            }
        }

        impl PoolItem for Awesome {
            fn new() -> Self {
                Self
            }
        }

        let pool: Pool<Awesome> = Pool::new();
        let item1 = pool.get();
        item1.do_thing();
        drop(item1);
    }
}
