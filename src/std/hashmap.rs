use std::collections::{HashMap, HashSet};

fn call(number: &str) -> &str {
    match number {
        "798-1264" => "We are sorry, the call cannot be completed as dialled. Please hang up and try again",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred. What can I get for you today?",
        _ => "Hi! Who is this again?"
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Debug)]
struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}, Password: {}", username, password);
    println!("Attempting logon...");

    let logon = Account {
        username,
        password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successfully logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Login failed!"),
    }
}

fn main() {
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1264");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // Takes a reference and returns Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
        _ => println!("Don't have Daniel's number"),
    }

    // `Hashmap::insert()` returns `None`
    // If the inserted value is new, Some(value) otherwise
    contacts.insert("Daniel", "164-6743");
    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", call(number)),
        _ => println!("Don't have Ashley's number"),
    }

    contacts.remove("Ashley");

    // `Hashmap::iter()` returns an iterator that yields
    // (&'a key, &'a value) pairs in arbitrary order
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }

    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "siebel.systems",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "Siebel Systems",
        email: "siebel@c3.ai",
    };

    accounts.insert(account, account_info);
    println!("Accounts: {:?}", accounts);
    try_logon(&accounts, "siebel.systems", "psassword123");
    try_logon(&accounts, "siebel.systems", "password123");

    /*
    * HashSet
    */

    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // `HashSet::insert()` returns false if
    // there was a value already present.
    // assert!(b.insert(4), "Value 4 is already in set B");
    b.insert(5);

    // If a collection's element type implements `Debug`
    // then the collection implements `Debug`
    // It usually prints its elements in the format `[elem1, elem2,...]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print [1,2,3,4,5] in arbitrary order
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // This should print [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Print [2,3,4] in arbitrary order
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // Print [2,5]
    println!("Symmetric Difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());


}