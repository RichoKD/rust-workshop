/**
* Shadowing
* You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one

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
