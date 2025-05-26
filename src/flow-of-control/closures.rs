use std::mem;
use std::iter;

 /**
     * Closures as input parameters
     */

     // A function which takes a closure as an argument and calls it.
     // <F> denotes that F is a "Generic type parameter"
     // FnOnce: The closure takes no input and returns nothing
    fn apply<F>(f: F) where F: FnOnce() {
        f();
    }

    // A function which takes a closure and returns an `i32`
    // Fn: The closure that takes a certain input (in this case `i32`) and returns an output(`i32`)
    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        f(3)
    }

// Define a function which takes a generic `F` argument bounded by `Fn` and calls it
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I am a function!");
}

// Closures as output parameters
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

fn main() {
    let outer_var = 10;
    
    // A regular function can't refer to variables in the enclosing environment
    // fn function(i: i32) -> i32 { i + outer_var }
    //TODO: Uncommenting the above function will cause a compilation error. The compiler suggests that we define a closure instead.

    // Closures are anonymous, here we are binding them to references.
    // Annotation is identical to function annotation but is optional.
    // as are the `{}` wrapping the body. These nameless functions are assigned to appropriately named variables.
    let closure_annotated = |x: i32| -> i32 { outer_var + x };
    let closure_inferred = |x| outer_var + x;

    // Calling the closures
    println!("closure_annotated(5) = {}", closure_annotated(5));
    println!("closure_inferred(5) = {}", closure_inferred(5));

    /**
     * Once closure's type has been inferred, it can not be inferred again with another type.
     * println!("cannot reuse closure_inferred with different type: {}", closure_inferred(42i64));
     */

     // A closure taking no arguments which returns i32
     // The return type is inferred
    let one = || 1;
    println!("one() = {}", one());

    let color = String::from("green");

    // A closure to print color which immediately borrows (`&`) color and 
    // stores the borrow and closure in the `print` variable. It will remain borrowed
    // until `print` is used the last time.
    let print = || println!("color: {}", color);
    print();

    // color can be borrowed immutably again, because the closure only holds a reference to it
    let _reborrow = &color;
    print();

    // A move or borrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment count could take either `&mut count` or `count`
    // but `&mut count` is less restrictive so it takes that. Immediately borrows count
    //
    // A mut is required on inc because a `&mut` is stored inside. Thus,
    // calling the closure mutates count which requires `mut`

    let mut inc = || {
      count += 1;
      println!("`Count`: {}", count);
    };

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    /**
     * `mem::drop` requires `T` so this must take by value. A copy type would copy into the closure leaving the original untouched.
     * A non-copy must move and so `movable` immediately moves into closure
     */
    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    let haystack = vec![1,2,3];
    let contains = move  | needle | haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&5));




    /*
    * Closure as an input parameter
    */
    let greeting = "Hello";
    // A non-copy type
    // `to_owned` creates owned data from a borrowed one
    let mut farewell = "goodbye".to_owned();

    let diary = || {
      // `greeting` is by reference: requires `Fn`
      println!("I said {}", greeting);

      // Mutation forces `farewell` to be captured by
      // mutable reference. Now requires `FnMut`
      farewell.push_str("!!!");
      println!("Then I screemed {}.", farewell);
      println!("Now I can sleep. zzzzz");

      // Manually calling drop forces `farewell` to
      // be captured by value. Now requires `FnOnce`
      mem::drop(farewell);

    };

    // Call the function which applies the closure
    apply(diary);

    // `double` satisfies `applies_to_3`'s straight bound
    let double = |x| 2*x;

    println!("3 doubled: {}", apply_to_3(double));

    let x = 7;

    // Capture x into an anonymous type and implement
    // `Fn` for it. Store it in `print`
    let print = || println!("{}", x);
    apply(print);

    let closure_ = || println!("I am a closure!");
    call_me(closure_);
    call_me(function);

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    // Examples in std library
    let vec1 = vec![1,2,3];
    let vec2 = vec![4,5,6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`
    println!("2 in vec1: {}", vec1.iter().any(|&x| x== 2));

    // `into_iter()` for vecs yields `i32`. No destructuring required
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x== 2));

    // `iter()` only borrows `vec1` and its elements, so they can be used again
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1 is: {}", vec1[0]);

    // `into_iter()` does not move `vec2` and its elements, so they cannot be used again
    // println!("First element of vec2 is: {}", vec2[0]);
    // println!("vec2 len: {}", vec2.len());
    //TODO: Uncomment two lines above and see the compiler errors

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));

    // `into_iter()` for arrays yields `i32`
    // println!("2 in array2: {}", array2.into_iter().any(|x| x== 2));


    // Searching through iterators
    let vector1 = vec![1,2,3];
    let vector2 = vec![4, 5, 6];

    // iter() for vecs yields i32
    let mut iter = vector1.iter();

    // into_iter() for vecs yields i32
    let mut into_iter = vector2.into_iter();

    // `iter()` for vecs yields `&i32` and we want to reference one of its items
    // so we have to destructure `&&i32` to `i32`
    println!("Find 2 in vector1: {:?}", iter.find(|&&x| x==2));
    // `into_iter()` for vecs yields `i32` and we want to reference one of
    // its items so we have to destructure `&i32` to `i32`
    println!("Find 2 in vector2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1,2,3];
    let array2 = [4,5,6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x==2));
    println!("Find 2 in array2: {:?}", IntoIterator::into_iter(array2).find(|&x| x==2));
}
