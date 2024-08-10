// Initial problem
//
// Fill the blank
// fn main() {
//     let mut s = __;
//     s.push_str("hello, world");
//     s.push('!');
//
//     assert_eq!(s, "hello, world!");
//
//     println!("Success!");
// }

// Fill the blank
pub fn main() {
    // the String static method is use for instantiating a new owned string.
    // The following methods can only work with a mut owned String
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}
