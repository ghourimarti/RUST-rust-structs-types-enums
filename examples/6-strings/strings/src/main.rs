// This is a simple example of how to use strings in Rust.
// Strings are a collection of characters.
// They are stored in a contiguous block of memory and can be accessed using an index.
// Strings are immutable by default, but can be made mutable using the mut keyword.

fn print_str(s: &str) {
    let new_string = format!("{} ---> other stuff here", s);
    println!("{}", new_string);
}

// means this function takes ownership of the String.
fn print_string(mut s: String) {
    println!("{}", s);
}

fn main() {
    println!("\n<=====================================>\n");
    let s = "hello, world!";
    print_str(s);

    // String is growable and mutable whereas str is not.
    // String is owned by the code that creates it
    let mut salutation = String::from("hello");
    print_string(salutation);
}
