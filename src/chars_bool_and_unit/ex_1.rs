// Make it work
use std::mem::size_of_val;
pub fn ex_1() {
    let c1 = 'a';
    println!("c1___{}", c1);
    let result = size_of_val(&c1);
    println!("result here___{}", result);
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    let c2_result: usize = size_of_val(&c2);
    println!("c2 result here___{}", c2_result);
    assert_eq!(size_of_val(&c2), 4);
    println!("Success!");
}
