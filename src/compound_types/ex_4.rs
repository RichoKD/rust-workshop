// Initial problem
//
// Fix all errors without adding newline
// fn main() {
//     let s = String::from("hello");
//     s.push(',');
//     s.push(" world");
//     s += "!".to_string();
//
//     println!("{}", s);
// }

// Fix all errors without adding newline
pub fn main() {
    // s has to be mutable to carry out mutating logic on it.
    let mut s = String::from("hello");
    s.push(',');

    // To add string slices to an owned string, use the `push_str` method.
    // `push` method is for adding a `char`` type
    s.push_str(" world");

    // Owned string cannot be added to another owned string because the operation is done on the stack.
    // string slices (`&str` type) is what is needed or using the format! macro if you really want to add two owned string.
    //
    // First approach
    // s += "!";
    //
    // Second approach
    // s += &"!".to_string();
    //
    // Easiest solution with format macro
    s = format!("{}{}", s, "!");

    println!("{}", s);
}
