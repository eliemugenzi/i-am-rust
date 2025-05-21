
fn main() {
    let n = 5;

    if n < 0 {
        println!("n is negative");
    } else if n == 0 {
        println!("n is zero");
    } else {
        println!("n is positive");
    }

    let big_n = (
        if n < 10 && n > -10 {
            10 * n
        } else {
            n / 2
        }
    );

    println!("big_n is {}", big_n);
}