// Fix the error without removing any code
pub fn main() {
    let s = String::from("Hello World");

    print_str(s.clone()); // Approach 1, clone the value (make a replica of the data) and pass it to the print_str() function
                          // print_str(&s); Approach 2, Reference the data stored in s (Borrowing) and pass the reference to the print_str() function

    println!("Hello world from main()");
    println!("{}", s);
}

fn print_str(s: String) {
    println!("Hello world from print_str()");
    println!("{}", s)
}
