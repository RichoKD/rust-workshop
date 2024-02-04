/*
   SCOPE - A scope is the range within the program for which the item is valid.
   Fix the error below with least amount of modification
*/
pub fn scope_fn() {
    // Ex 3a
    let x: i32 = 10;
    let y: &str = "fr";
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
    call_define_x()
}

// Ex 3b
// Fix the error with the use of define_x
fn call_define_x() {
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x: &str = "hello";
    x.to_string()
}
