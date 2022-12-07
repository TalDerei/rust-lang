// Generics are used for generalizing types and reducing code duplication

// A concrete type `A`
pub struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`
// Therefore, `Single` is a concrete type, and `A` is defined as above
pub struct Single(pub A);

// Here, `<T>` precedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top
pub struct SingleGen<T>(pub T);

// `<T>` Must precede the type to remain generic
impl<T> SingleGen<T> {}
