```Documentation```
Rust by Example: https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
Rust Book: https://doc.rust-lang.org/book/ch10-00-generics.html

``` Static Typing ```
Rust is a statically-typed language, where variables are known at compile time. 

```Mutability```
Variable bindings are immutable by default, and can be overriden with `mut` modifier.
Mutable data can be mutable borrowed using &mut T. This is known as a mutable reference 
and gives read/write access to the borrower. &T is an immutable reference. You can have 
any number of immutable borrows at the same time, but only one mutable borrow. "Ref" borrows 
reference fields of a struct / tuple (ie. ref c = d is the same as let c = &d);

Cannot have an immutable and mutable reference to the same data to avoid data races. This is enforced by the borrow checker for enabling safe concurrency. If this wasn’t the case, then many mutable references would be allowed. Mutable references are meant to be exclusive references, while immutable references are shared references. The concept of “Reference Counting” (RC) gives shared references to data, but it cannot be mutable. To have shared mutability, you need to use an interior mutability data structure like RefCell. Instead of a garbage collection, RC uses an internal counter to track references that’s incremented (clone) and decremented (drop). These concepts prevent memory leaks and concurrency data races in the compiler itself!

```Modules```
A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.
By default, modules have private visibility. 

```Crates```
A crate is a compilation unit (i.e. files). It can be compiled into a binary (main.rs) or library (lib.rs).
Use the 'external' keyword to import a library. 

```Cargo```
Rust package and dependency manager with access to crates.io (rust package registry).

```RAII```
"Resource Acquisitiion Is Initialization" ensures that whenever an object goes out of scope, 
the destructor is called and its *owned* resources (e.g. Box<T> owns memory in the heap) are freed. 
This shields against memory leaks!

```Ownership```
Variables are in charge of freeing their own resources, so resources can only have one owner. 
Transfering ownership of resources between variables is known as a "move". 

Rust ownership and borrow checker: for some type T you have — T, &T (shared / immutable reference), &mut T (exclusive / mutable reference).References (&T and &mut T) have lifetimes: cannot outlive T. Resources only have a single owner, so owner frees only once. This is checked at compile time by the borrow checker to prove that program does not have any data races. 

```References and Borrowing```
Borrowing is passing objects by reference (&T) instead of passing by value (T).
Accessing data without taking ownership of it, gauranteed and enforced by the borrow checker. 
Mutable data can be mutabley borrowed (&mut T) known as a "mutable reference" giving read/write 
access to the borrow. You may have one or the other of these two kinds of borrows, but not both 
at the same time: (1) one or more references (&T) to a resource, (2) exactly one mutable reference 
(&mut T). This is similair to the definition of a data race: "There is a ‘data race’ when two or 
more pointers access the same memory location at the same time, where at least one of them is writing, 
and the operations are not synchronized."

```Liftimes```
Lifetime is a construct by the compiler / borrow checker to ensure all borrows are valid.
Borrow checker ensures that references always point to valid objects (ie. avoid danging
pointers). Specifically, a variable's lifetime begins when it is created and ends when it 
is destroyed.

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
1. Integration Testing: Unit tests are testing one module in isolation at a time: they're small 
and can test private code. Integration tests are external to your crate and use only its public 
interface in the same way any other code would. Their purpose is to test that many parts of your 
library work correctly together.
1. Document Testing

```Unsafe Operations```
Unsafe annotations in Rust are used to bypass protections put in place by the compile
- Dereferencing raw pointers (similiar to references)
- Calling functions or methods which are `unsafe`
- Accessing or modifying static mutable variables
- Implementing unsafe traits

```FFI (Foreign Function Interface)```
Mechanism that allows Rust to call code written in C++, for example. 

```Traits```
Traits are similair to interfaces, defining a set of common behaviors / methods 
for different types. To implement a trait for a particular type, you use the `impl` keyword.
There are existing trait implementation via the “derive” attribute (e.g copy, debug, etc.).

```Generics```
Generics are used for generalizing types and reducing code duplication.

```Error Handling```
Enforce error handling via print statements, panic, unimplemented keyword, option and result (rs).

----------------------------------------------------------------------------------------------------------------
Questions: 

Q. What's the difference between self, &self, &mut self, Self?
`self` refers to current instance of an object that takes ownership, `&self` is an immutable borrowed
reference of the current instance and `&mut self` can further mutate it. `Self` refers
to the current object (i.e. struct, enum, trait).

Q. Can you have memory leaks in Rust?
Yes, if you use `unsafe` keyword and try dereferencing raw pointers. But generally compared to C / C++,
there are no seg-faults, no null pointers, no data races, no buffer overflows, etc. Microsoft research 
found that 70% of security vunerabilities in it's applications were memory related, as an example! 
Also, Rust doesn't have garbage collection and small runtime, leading to small memory footprints.

Q. Raw points vs references?
Raw pointers * and references &T function similarly, but references are always safe because they point 
to valid data because of the borrow checker. You can deterrence a raw pointer through the ‘unsafe’ block.

Q. Does Rust use LLVM?
Yes, Rust uses LLVM as a backend to compile rust. The target can be LLVM IR or binary machine code. 
Other backends support GCC compilation, or something like "rust-gpu" backend to generate SPIR-V for GPU. 

Q. Does Rust work with traditional tools and languages?
Yes, Rust is compatible with other languages (e.g. C++) through zero-cost FFI, WASM support, and
traditional tooling like LLVM sanitizers and optimizations, GDB debugger, Valgrind, etc. The built-in
toolchain 'rustup' and compiler 'rust' also have many features. 

Q. So how do we enable concurrent reads and writes to a shared data structure?
Rust will force us to use synchronization. Rust will not allow you to access shared value without “mutex”. A mutex is a lock that provides safe mutations on shared data. R/W locks are extensions of this concept. 

Q. What is unsafe keyword?
In unsafe code, you can dereference raw pointers. E.g. *mut T is a raw pointer (with no lifetime) to T. It can be turned into &mut T, but doing so is unsafe. In unsafe code, you’re responsible for not adding data races. If your code crashes, audit this section of the code. 

Q. Why is Rust binary size so large?
Rust uses monomorphization and static linkink, leading to large binaries. Flags passsed into the cargo.toml file
can dramatically reduce the binary size (e.g. debug binaries can be 30% larger than release binaries):
https://github.com/johnthagen/min-sized-rust