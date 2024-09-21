// Initial Problem
//

// Fix the error with at least two solutions
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }

pub fn main() {
    // we can Box str type since the Box type stores data on the heap
    let s: Box<str> = "hello, world".into();

    // the greetings function signature accepts &str as parameter.
    // we can use the reference symbol to get a reference to str in the heap which is a valid argument for greetings function
    greetings(&s)
}

fn greetings(s: &str) {
    println!("{}", s)
}
