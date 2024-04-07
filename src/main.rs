fn main() {
    println!("Hello, World");

    calc_hello("Rick".to_string())
}

// calc_hello() is a function that accepts a name as string and prints a hello message that includes the name passed into the function.
fn calc_hello(name: String) {
    println!("Hello, {}", name);
}
