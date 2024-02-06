/**
* Shadowing
* You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one
* Ex 4a

*/

// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
pub fn shadow_fn() {
    let x: i32 = 5;
    {
        assert_eq!(x, 5);
        println!("1st x here is: {}", x); // prints 5
        let x = 12;

        println!("2nd x here is: {}", x); // prints 12
        assert_eq!(x, 12);

    }

    let x = 42;
    assert_eq!(x, 42);
    println!("{}", x); // Prints "42".
}

/**
 * 
* Ex 4a
* Remove a line in the code to make it compile
 * 
 */
pub fn shadow_fn_b() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let x = x; 
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

