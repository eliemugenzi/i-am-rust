
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}
fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: if `let` destructures number into Some(i), evaluate the bloock (`{}`)
    if let Some(i) = number {
        println!("Matched {}!", i);
    }

    // If you need to specify a failure, use an else
    if let Some(i) = letter {
        println!("Matched {}!", i);
    } else {
        // Destructure failed. Change to the failure case
        println!("No match");
    }

    // Provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {}!", i);
    } else if i_like_letters {
        println!("I like letters");
    } else {
        println!("No match");
    };

    /**
     * if let can be used to match any enum value
     */

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(130);

    // Variable a matches the enum Foo::Bar
    if let Foo::Bar = a {
        println!("a is Bar");
    }

    if let Foo::Qux(i) = c {
        println!("c is Qux with value {}", i);
    }

    if let Foo::Qux(value @ 130) = c {
        println!("c is Qux with value {}", value);
    }


}