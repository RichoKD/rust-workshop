pub fn main() {
    // Use as many approaches as you can to make it work
    let x: String = String::from("Hello world");
    let y: &String = &x;
    println!("{}, {}", x, y);
}