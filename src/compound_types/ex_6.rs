// Initial problem

// Fix errors without removing any line
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1 + s2;
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s1);
// }

// Fix errors without removing any line
pub fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");

    // The addition has to be re-arranged. You can only use the + operand to add an a string slice to an owned string (String + &str).
    // Also, we need to use s1 after the addition. We pass s1 as a reference to the operation and make s2 the first variable in the addition.
    // Remember rust ownership system.
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
