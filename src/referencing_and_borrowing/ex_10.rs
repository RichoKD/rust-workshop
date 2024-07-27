
// Comment one line to make it work
pub fn main() {
  let mut s = String::from("hello, ");

  let r1 = &mut s;
  r1.push_str("world");
  // Rust does not allow multiple mutable references to the same data at the same time. That is why i comment line 9 let r2 = &mut s; and change r2 to r1.
  // let r2 = &mut s;
  r1.push_str("!");
  
  println!("{}",r1);
}