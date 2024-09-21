pub fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let a = String::from("Hello world");
    // Convert String to Vec
    let _s = a.clone().into_bytes(); // Clone the value (make a replica of the data) and pass the cloned value to the into_bytes() method call
    a
}
