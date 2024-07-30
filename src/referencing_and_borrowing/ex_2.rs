pub fn main() {
  let x = 5;
  let y = &x;

  // Modify this line only
  // By dereferencing y using *, we compare the value behind the reference with 5, ensuring the assertion passes
  assert_eq!(5, *y);

  println!("Success!");
}