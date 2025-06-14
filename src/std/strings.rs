use std::str;
fn main() {
    // (All the type annotations are superfluous)
    // A reference to a string allocated in read-only memory
    let pangram: &'static str = "The quick brown fox jumps over the lazy dog";
    println!("{}", pangram);

    // Iterate over words in reverse, no new string is allocated
    println!("Words in reverse:");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Copy chars into a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a char at the end of string
        string.push(c);
        // Insert a string at the end of string
        string.push_str(", ");
    }

    // The trimmed string is a slice to the original string,
    // hence no new allocation is performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");
    println!("Bob: {}", bob);
    println!("Alice: {}", alice);

    // You can use escapes to write bytes by their hexadecimal values...
    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you doing\x3F (\\x3F means?) {}", byte_escape);

    // ...or Unicode code points.
    let uncode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} is called {}", uncode_codepoint, character_name);

    let long_string = "String literals\
    can span multiple lines.\
    The linebreak and indentation here -> \
    <- can be escaped too!";
    println!("{}", long_string);

    let raw_str = r"Escape don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 255 #s
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // Note that this is not a `&str`
    let bytestring: &[u8;21] = b"This is a byte string";
    // Byte arrays don't have the Display trait, so printing them is a bit limited
    println!("A byte string{:?}", bytestring);

    // Byte strings can have byte escapes...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ... but no Unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

}