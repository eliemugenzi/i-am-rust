use std::fs::File;
use std::path::PathBuf;
use std::io::Result;

struct TempFile {
    path: PathBuf,
    file: File,
}
struct Droppable {
    name: &'static str,
}

impl TempFile {
    fn new(path: PathBuf) -> Result<Self> {
        // Note: File::create() will overwrite existing files
        let file = File.create(&path)?;
        Ok(Self { path, file })
    }
}

// When TempFile is dropped;
// 1. First, the File will be automatically closed (Drop for File)
// 2. Then our drop implementation will remove the file
impl Drop for TempFile {
    fn drop(&mut self) {
        // Note: File is already closed at this point
        if let Err(e) = std::fs::remove_file(&self.path) {
            eprintln!("Error removing file: {}", e);
        }

        println!("> Dropped temporary file: {:?}", self.path);

    }
}

// This trivial implementation of `drop` adds a print to console
// The Drop trait is a special trait in Rust that defines what happens when a value goes out of scope
// It's automatically called by the compiler when a value is no longer needed
// This is part of Rust's RAII (Resource Acquisition Is Initialization) pattern
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // Block A
    {
        let _b = Droppable { name: "b" };

        // Block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting Block B");
        }

        println!("Just exited Block B");
        println!("Exiting Block A");
    }

    println!("Just exited Block A");

    // Variable can be manually dropped using the `drop` function
    // This is the std::mem::drop function from the standard library, not the drop method from the Drop trait
    // It's imported automatically through the prelude
    // The std::mem::drop function forces a value to be dropped before the end of its scope
    // When drop(_a) is called, it moves _a into the drop function, which then immediately drops it,
    // calling the Drop::drop implementation we defined above
    drop(_a);
    println!("Exiting main");
    // _a won't be dropped again here, because it already has been manually dropped
}
