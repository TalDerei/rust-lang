```Documentation```
Rust by Example: https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
Rust Book: https://doc.rust-lang.org/book/ch10-00-generics.html

``` Static Typing ```
Rust is a statically-typed language, where variables are known at compile time. 

```Mutability```
Variable bindings are immutable by default, and can be overriden with `mut` modifier.

```Modules```
A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.
By default, modules have private visibility. 

```Crates```
A crate is a compilation unit. It can be compiled into a binary (main.rs) or library (lib.rs).

```RAII```
"Resource Acquisitiion Is Initialization" ensures that whenver an object goes out of scope, 
the destructor is called and its owned resources are freed. This shields against memory leaks!

```Ownership```
Variables are in charge of freeing their own resources, so resources can only have one owner. 

```Borrowing```
Accessing data without taking ownership of it, gauranteed by the borrow checker. Mutable 
data can be mutabley borrowed (&mut T) known as a "mutable reference" giving read/write 
access to the borrow.

```Liftimes```
Lifetime is a construct that the compiler / borrow checker to ensure all borrows are valid.
Specifically, a variable's lifetime begins when it is created and ends when it is destroyed.

```Std Library Types```
- Dynamic vectors: [1, 2, 3],
- Optional types: Option<i32>,
- Error handling types: Result<i32, i32>,
- Head allocated pointers: Box<i32>,

```Box, stack and heap```
All rust values are stack allocated by default. Values can be boxed (allocated on the heap)
by creating Box<T>, a smart pointer to heap allocated value of type T. When a box goes out 
of scope, its destructor is called, the inner object is destroyed, and the memory on the 
heap is freed. Boxed values can be dereferenced using the * operator; this removes one layer 
of indirection.

RC (Reference Counting) is used when multiple owners are needed. Rc keeps track of the number 
of the references which means the number of owners of the value wrapped inside an Rc.
Reference count of an Rc increases by 1 whenever an Rc is cloned, and decreases by 1 whenever 
one cloned Rc is dropped out of the scope. When an Rc's reference count becomes zero (which 
means there are no remaining owners), both the Rc and the value are all dropped. Cloning an Rc 
never performs a deep copy. Cloning creates just another pointer to the wrapped value, and 
increments the count.

```Testing```
1. Unit Testing: Most unit tests go into a tests mod with the #[cfg(test)] attribute. 
Test functions are marked with the #[test] attribute.
2. Integration Testing: Unit tests are testing one module in isolation at a time: they're small 
and can test private code. Integration tests are external to your crate and use only its public 
interface in the same way any other code would. Their purpose is to test that many parts of your 
library work correctly together.
3. Document Testing

```Unsafe Operations```
Unsafe annotations in Rust are used to bypass protections put in place by the compile
- Dereferencing raw pointers (similiar to references)
- Calling functions or methods which are `unsafe`
- Accessing or modifying static mutable variables
- Implementing unsafe traits

```FFI (Foreign Function Interface)```
Mechanism that allows Rust to call code written in C++, for example. 