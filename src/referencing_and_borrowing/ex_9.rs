
// This code has no errors!
pub fn main() {
  let mut s = String::from("hello, ");

  // The &mut keyword is used to create a mutable reference to s
  borrow_object(&mut s);
  
  s.push_str("world");

  println!("Success!");
}

fn borrow_object(s: &String) {}