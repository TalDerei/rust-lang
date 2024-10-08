```Copy Trait```
By default, variable bindings take ownership of the variable they're binding, and when they go out of scope, the memory and bound resources are freed. Variable bindings have 'move semantics' by default, e.g. `x` has moved into `y`, and so x cannot be used. By implementing the 'Copy' trait, #[derive(Debug, Copy, Clone)], or manually implementing 'Copy' for the struct, e.g. `impl Copy for MyStruct { }`, the type now implements Copy and instead uses 'copy semantics' to perform a full copy. Instead of `x` being moved into `y`, `y` is a copy of `x`, and `x` can be accessed in the print function. Under the hood, both a copy and move can result in bit-wise copy in memory. Generally speaking, if your type can implement 'Copy', it should!

The copy trait is only implementable by data types that can be put on the stack, and because e.g. vec must go on the heap, it cannot implement copy. 'Copy' is stack-based and cheap. 

```Clone Trait```
In #[derive(Debug, Copy, Clone)], 'Clone' is also required since it is a supertrait of 'Copy'. 'Clone' must be derived in order to explicitely invoke .clone() on structs. Copy is implicit and an inexpensive bit-wise copy, while Clone is always explicit and may or may not be expensive.

The clone trait is implementable by data types that can be put on the stack and heap. Clone is more general than copy, so everything that can be a copy can be a clone as well. Clone is heap-based and expensive. 