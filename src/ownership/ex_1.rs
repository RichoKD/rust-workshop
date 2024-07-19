pub fn main() {
    // Use as many approaches as you can to make it work
    // let x: String = String::from("Hello world"); 
    // let y: &String = &x; // approach 1 using references(Borrowing)

    // let y = x.clone(); // approach 2 using clone method (making a replica of the data)
    // println!("{}, {}", x, y);

    let x: &str = "Hello World"; // approach 3 using string slices (Which are stored on the stack and can be copied.)
    let y = x;
    println!("{}, {}", x, y);

    // let x = &String::from("Hello World");
    // let y = x;
    // println!("{}, {}", x, y);
}