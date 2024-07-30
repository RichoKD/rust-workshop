pub fn main() {
  let mut s = String::from("hello, ");

  let r1 = &mut s;
  // Uncomment the line below to make a compiler error
  // let r2 = &mut s;

  r1.push_str("world");
  r1.push_str("!");

  println!("{}", r1);
}