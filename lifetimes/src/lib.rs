// Struct generic over lifetime 'a', tying the lifetime of the struct
// to the lifetime of the inner slice. The struct can only live as long
// as the inner slice is valid. 
struct MyIterWrapper<'a, T> {
    slice: &'a [T],
}

// Types 'a and T are generic parameters for the implementation
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
            assert_eq!(*elem, collection[index])
        }
    }
}
