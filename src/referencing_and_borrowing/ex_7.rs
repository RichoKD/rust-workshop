
// Remove something to make it work
// Don't remove a whole line !
pub fn main() {
  let mut s = String::from("hello");

  let r1 = &mut s;
  // The comment on line 9, // let r2 = &mut s;, is there because Rust does not allow multiple mutable references to the same data at the same time.

  // let r2 = &mut s;

  // To fix this error, we can use the clone method to create a copy of the value behind the mutable reference r1 and assign it to r2.
  let r2 =  r1.clone();

  println!("{}, {}", r1, r2);

  println!("Success!");
}


