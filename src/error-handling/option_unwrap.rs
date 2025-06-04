
fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Enjoy your drink!"),
        Some(inner) => println!("{}? How nice.", inner),
        None => panic!("No drink? Oh well."),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("Ahaaa!");
    }
    println!("I love {}s", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;
    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}