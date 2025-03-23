mod lib;

fn main() {
    println!("Jenkins Rust Test Runner Initialized!");
    let result = lib::add(10, 5);
    println!("10 + 5 = {}", result);
}
