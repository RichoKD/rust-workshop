
// Fix error
pub fn main() {
  let mut s = String::from("hello, ");

  // borrow_object takes a mutable reference to s, allowing it to modify the string. That is why i added &mut to s

  borrow_object(&mut s);

  println!("Success!");
}

fn borrow_object(s: &String) {}