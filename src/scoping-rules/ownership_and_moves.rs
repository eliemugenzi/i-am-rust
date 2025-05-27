
// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // c is destroyed and the memory freed
}

#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>
}

// impl Drop for Person {
//     // Error! cannot move out of a type which implements the `Drop` trait
//     // fn drop(&mut self) {
//     //     println!("Dropping the person struct {:?}", self);
//     // }
//     // TODO^ Try uncommenting these lines
// }


fn main() {
    // _Stack_ allocated integer
    let x = 5u32;

    // Copy `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {} and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // Move `a` into `b`
    let b = a;

    // The pointer address of `a` is copied (not the data) into `b`
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it

    // Error! a can no longer access the data, because it no longer owns the
    // heap memory
    // println!("a contains: {}", a);
    // TODO^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action
    // would result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    // println!("b contains: {}", b);
    // TODO^ Try uncommenting this line


    // Mutability
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    // *immutable_box = 4;

    // Move the box, changing the ownership( and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);

    // Partial moves
    let person = Person { name: String::from("Alice"), age: Box::new(20) };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}
