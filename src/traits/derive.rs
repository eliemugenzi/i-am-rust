// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a struct that can be debugged and printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        Centimeters(self.0 as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct that has no additional attributes
struct Seconds(i32);
fn main() {
    let _one_second = Seconds(1);

    // Error! `_one_second` cannot be printed because it has no `Debug` implementation
    // println!("One second looks like: {:?}", _one_second);
    // TODO^ Try uncommenting this line

    // Error! Seconds cannot be compared because it has no `PartialEq` implementation
    // let _this_is_true = (_one_second == _one_second);
    // TODO^ Try uncommenting this line

    let foot = Inches(12);
    println!("One foot equals: {:?}", foot);
    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("One foot is {} than one meter", cmp);
}