
use crate::List::*;
/*
* Crerate an enum to classify a web event. Note how both names and type information together specify the variant:
* `PageLoad != PageUnload` and `KeyPress(char)!= paste(String)`
*/
enum WebEvent {
    // An `enum` variant may either be `unit-like`
    PageLoad,
    PageUnload,
    // like tuple structs
    KeyPress(char),
    Paste(String),
    // or c-like structures
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        // Destructure c from inside the enum variant
        WebEvent::KeyPress(c) => println!("Key pressed: {}", c),
        WebEvent::Paste(s) => println!("Pasted: {}", s),
        // Destructure Click into x and y
        WebEvent::Click { x, y } => println!("Clicked at: ({}, {})", x, y),
    }
}

// Type aliases
enum NumbersOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

type Operations = NumbersOperation;

impl NumbersOperation {
    fn run(&self, x: i32, y: i32)-> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Multiply => x * y,
            Self::Divide => x / y,
        }
    }
}

enum Stage {
    Beginner,
    Intermediate,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xFF0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// 2. Linked List

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
    // Create a new empty list
    fn New() -> List {
        // Nil has the same type as List
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn Prepend(self, elem: u32) -> List {
        // Cons also has type List
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> i32 {
        match self {
            // If the list is empty, return 0
            Nil => 0,
            // If the list is not empty, return 1 + length of the rest of the list
            Cons(_, tail) => 1 + tail.len(),
        }
    }

    fn stringify(&self) -> String {
        match self {
            // If the list is empty, return an empty string
            Nil => format!("Nil"),
            // If the list is not empty, return the string representation of the first element + the string representation of the rest of the list
            Cons(head, tail) => format!("{}, {}", head, tail.stringify()),
        }
    }
}



fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste(String::from("hello"));
    let clicked = WebEvent::Click { x: 100, y: 200 };
    let loaded = WebEvent::PageLoad;
    let unloaded = WebEvent::PageUnload;
    inspect(pressed);
    inspect(pasted);
    inspect(clicked);
    inspect(loaded);
    inspect(unloaded);


    /**
     * Explicitly `use` each name so they are available without manual scoping
     */

    use crate::Stage::{Beginner, Intermediate, Advanced};

    // Automatically `use` each name inside `Role`
    use crate::Role::*;

    // Equivalent to `Stage::Beginner`
    let stage = Beginner;
    let role = Student;

    match stage {
        Beginner => println!("Beginner"),
        Intermediate => println!("Intermediate"),
        Advanced => println!("Advanced"),
    }

    match role {
        Student => println!("Student"),
        Teacher => println!("Teacher"),
    }

    // `enums` can be cast as integers
    println!("Number::Zero = {}", Number::Zero as i32);
    println!("Number::One = {}", Number::One as i32);

    println!("Color::Red = #{:06x}", Color::Red as i32);
    println!("Color::Green = #{:06x}", Color::Green as i32);
    println!("Color::Blue = #{:06x}", Color::Blue as i32);

    // Linked List
    let mut list = List::New();
    list = list.Prepend(1);
    list = list.Prepend(2);
    list = list.Prepend(3);

    println!("List length: {}", list.len());
    println!("List: {}", list.stringify());
}