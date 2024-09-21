pub fn main() {
    // Fix error by modifying this line
    // I fixed this error by adding mut to s so that it can be be borrowed mutably by borrow_object()
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
