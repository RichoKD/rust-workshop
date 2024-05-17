use std::mem::size_of_val;

// Make it work with two ways
pub fn main() {
    let mut x = 1;
    let v = {
        // let mut x = 1;
        x += 2
        // return x;
    };
 
    assert_eq!(v, ());
    // let result: usize = size_of_val(&v);
    assert_eq!(v, v);
 
    println!("Success!");
 }