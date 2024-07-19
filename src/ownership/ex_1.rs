pub fn main() {
    // Use as many approaches as you can to make it work
    // let x: String = String::from("Hello world"); 
    // let y: &String = &x; // approach 1

    // let y = x.clone(); // approach 2
    // println!("{}, {}", x, y);

    let x: &str = "Hello World"; // approach 3
    let y = x;
    println!("{}, {}", x, y);
}