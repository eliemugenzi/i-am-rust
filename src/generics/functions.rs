// A concrete type A
struct A;
struct S(A); // Concrete type S
struct SGen<T>(T); // Generic type `SGen`

// In defining the type `Single`, the first use of `A` is not proceded by `<A>`.
// Therefore, Single is a concrete type, and `A` is defined as above
struct Single(A);

// Here, <T> procedes the first use of `T`, so `SingleGen` is a generic type.
// Because the type parameter `T` is generic, it could be anything, including
// the concrete type `A` defined at the top
struct SingleGen<T>(T);

// The following functions all take ownership of the variable passed into
// them and immediately go out of scope, freeing the variable

// Define a function reg_fn that takes an argument `_s` of type `S`.
// This has no `<T>` so this is not a generic function
fn reg_fn(_s: S) {}

// Define a function `gen_spec_t` that takes an argument `_s` of type `SGen<T>`
// It has been explicitly given the type parameter A, but because A has not
// been specified as a generic type parameter for `gen_spec_t`, it is not generic
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SGen<i32>`
// It has been explicitly given the type parameter `i32`, which is a specific type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function generic that takes an argument `_s` of type `SGen<T>`
// Because SGen<T> is preceded by <T>, this function is generic over T
fn generic<T>(_s: SGen<T>){}

// Implementation
struct Z; // Concrete type S
struct GenericVal<T>(T); // Generic type GenericVal

// impl of GenericVal where we explicitly specify type parameters
impl GenericVal<f32> {} // Specify f32
impl GenericVal<Z> {} // Specify S as defined above
impl <T> GenericVal<T> {} // <T> must precede the type to remain generic

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// imp, of GenVal for a generic type T
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    // `Single` is concrete and explicitly takes `A`
    let _s = Single(A);

    // Create a variable _char of type SingleGen<char>
    // and give it the value `SingleGen('a')`
    // Here, `SingleGen` has a type parameter explicitly specified.

    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified:
    let _t = SingleGen(A); // uses `A` defined at the top
    let _i32 = SingleGen(6); // uses `i32`
    let _char = SingleGen('a'); // Uses `char`

    // Using the non-generic functions
    reg_fn(S(A)); // Concrete type.
    gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`

    // Explicitly specified type parameter `char` to `generic()`
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`
    generic(SGen('c'));

    let x = Val {val: 3.0};
    let y = GenVal{ gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());

}
