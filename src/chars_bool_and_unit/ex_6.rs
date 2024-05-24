// Unit

// Modify `4` in assert to make it work
use std::mem::size_of_val;
pub fn main() {
    let unit: () = ();

    let size_result: usize = size_of_val(&unit);
    println!("size of unit: {:?}", size_result);
    assert!(size_of_val(&unit) == 0);

    println!("ex_6 result: success!");
}
