pub fn main() {
  let mut s = String::from("hello, ");

  // Fill the blank to make it work
  // By using &mut, we ensure that only one mutable reference to s exists at a time, preventing data races and other concurrency issues
  let p = &mut s;
  
  p.push_str("world");

  println!("Success!");
}