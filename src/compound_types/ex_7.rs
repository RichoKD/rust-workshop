// Initial problem

// Fix error with at least two solutions
// fn main() {
//     let s = "hello, world";
//     greetings(s)
// }

// fn greetings(s: String) {
//     println!("{}", s)
// }

// Fix error with at least two solutions
pub fn main() {
    let s = "hello, world";

    // Solution 1: Use the String::from() static method which takes an owned string and converts it to a owned string.
    // let s= String::from(s);

    // Solution 2: Call the to_string string slice method to convert a string slice to an owned string.
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}", s)
}
