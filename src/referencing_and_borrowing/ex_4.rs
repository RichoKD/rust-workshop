// Fix error
pub fn main() {
  let mut s = String::from("hello, ");

  // borrow_object takes a mutable reference to s, allowing it to modify the string. That is why i added &mut to s. Now s is now "hello, world"
  push_str(&mut s);

  println!("Success!");
}

fn push_str(s: &mut String) {
  s.push_str("world")
}