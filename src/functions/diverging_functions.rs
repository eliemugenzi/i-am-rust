// Diverging functions never return. They are marked using !, which is an empty type
fn foo() -> ! {
    panic!("This call never returns");
}

// This function returns as usual, although there is no information in the return value
fn some_fun() {
    ()
}

fn main() {
    let a:() = some_fun();
    println!("This function returns and you can see this line.");
}