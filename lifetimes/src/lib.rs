// Struct generic over lifetime 'a', tying the lifetime of the struct
// to the lifetime of the inner slice. The struct can only live as long
// as the inner slice is valid. 
struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

// Types 'a and T are generic parameters for the implementation.
// This is an immutable, non-exclusive iterator.
impl<'a, T> Iterator for MyIterWrapper<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        // Retrieve first element
        let element = self.slice.get(0);
        // Set self.slice equal to other elements
        self.slice = &self.slice[1..];
        // Return first element
        element
    }
}

// Marking the struct as 'mutable' creates an exclusive, single reference. 
struct MyMutableIterWrapper<'iter, T> {
    slice: &'iter mut [T],
}

impl <'iter, T> Iterator for MyMutableIterWrapper<'iter, T> {
    type Item = &'iter mut T; 
    
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        // Double borrow (ie. pointer) notation to avoid lifetime conflicts. 
        // In the next line, we're using variable shadowing. 
        let slice = &mut self.slice;        
        // Replaces self.slice with empty, and previously self.slice is contained inside slice.
        // Now we can operate on self.slice with lifetime 'iter
        let slice = std::mem::replace(slice, &mut []);
        let (first, rest) = slice.split_first_mut()?;
        self.slice = rest;
        Some(first)        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let collection = vec![1,2,3,4];
        let wrapper = MyIterWrapper {
            slice: &collection[..],
        };
        
        for (index, elem) in wrapper.enumerate() {
            assert_eq!(*elem, collection[index]);
        }

        let mut collection = vec![1,2,3,4];
        let wrapper = MyMutableIterWrapper {
            slice: &mut collection[..],
        };
        
        for (index, elem) in wrapper.enumerate() {
            *elem = *elem + 1;
        }

        assert_eq!(collection.get(0), Some(&2));
    }
}