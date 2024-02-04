/*
    SCOPE - A scope is the range within the program for which the item is valid. 
    Fix the error below with least amount of modification
 */
pub fn scope_fn() {
    let x: i32 = 10;
    let y: &str = "fr";
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}


