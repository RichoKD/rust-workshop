// Don't modify code in main!
pub fn main() {
    println!("Ownership belongs to s1 variable");
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("Ownership moved to s2 variable");
    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("Ownership moved to take_ownership");
    println!("{}", s);
    s
}