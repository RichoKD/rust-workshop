// make the necessary variable mutable
pub fn main() {
    let s = String::from("Hello ");

    // Added mut to s1 and .clone() to s.
    let mut s1 = s.clone();

    s1.push_str("World!");

    println!("Success!");
}
