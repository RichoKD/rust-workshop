// Make it work with two ways
pub fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };
 
    assert_eq!(v, 3);
    // let result: usize = size_of_val(&v);
    assert_eq!(v, v);
 
    println!("ex_1 result: Success!");
 }