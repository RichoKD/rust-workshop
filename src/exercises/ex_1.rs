// Ex 1 UNUSED VARIABLES
// Fix the error below with least amount of modification to the code
pub fn unused_var() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
