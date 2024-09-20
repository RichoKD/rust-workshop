// Initial problem

// Use two approaches to fix the error and without adding a new line
// fn main() {
//     let s = "hello, world".to_string();
//     let s1: &str = s;

//     println!("Success!");
// }

// Use two approaches to fix the error and without adding a new line
pub fn main() {
    let s = "hello, world".to_string();

    // Solution 1: borrow s.
    // let s1 = &s;

    // Solution 2: use to `as_str` string slice method to get a borrowed string from an owned string.
    let s1: &str = s.as_str();

    println!("Success!");
}
