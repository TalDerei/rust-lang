use std::alloc::{self, dealloc};
use std::ptr::NonNull;

pub struct MyVec<T> {
    // NonNull<T> is similiar to non-zero raw, mutable pointer (*mut T)
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if std::mem::size_of::<T>() == 0 {
            panic!("Type T is equal to 0!")
        }
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).unwrap();
            // Explicitely cast as *mut T, where the layout (block of memory) is
            // hardcoded to be 4 * size_of<T> and size_of<T> > 0.
            // Returns a pointer to newly allocated memory on the heap. 
            // 'Unsafe' keyword required since interacting with raw pointers. 
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            // Wrap ptr *mut T in NonNull<T>
            let ptr = NonNull::new(ptr).expect("Could not allocate memory");
            // Assign pointer to item being pushed
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot reach memory location");
            assert!(offset < isize::MAX as usize, "wrapped isize");
            // Offset cannot wrap around and pointer to valid memory
            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
            }
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);
            let new_capacity = self
                .capacity
                .checked_mul(2)
                .expect("Cannot allocate memory");
            let size = std::mem::size_of::<T>() * self.capacity;
            let align = std::mem::size_of::<T>();
            size.checked_add(size % align)
                .expect("Cannot allocate memory");
            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                // Check ptr is NonNull
                let ptr = NonNull::new(ptr as *mut T).expect("Coudl not reallocate memory");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        // Returns a pointer, requiring dereferencing the pointer
        // and returning a reference wrapped in 'Some'. BUT,
        // dereferencing the pointer drops the pointer. 
        Some(unsafe {
            &*self.ptr.as_ptr().add(index)
        })
    } 
}

// Destructor to fix memory leaks in Valgrind
impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));        
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::size_of::<T>(),
            );
            dealloc(self.ptr.as_ptr() as *mut u8, layout); // explicit type cast _ instructs compiler to figure out cast
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_vec() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.get(3), Some(&4));
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
