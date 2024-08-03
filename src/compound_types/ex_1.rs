// Initial problem
//
// Fix error without adding new line
// fn main() {
//     let s: str = "hello, world";

//     println!("Success!");
// }

// Solution

pub fn main() {
    let s: &str = "hello, world"; // we can only use str type on it own as a borrowed type.

    println!("Success!");
}
