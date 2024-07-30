pub fn main() {
  let x = Box::new(5);
  
  // `mut` was added to y and .clom() was added to x
  let mut y = x.clone();   // update this line, don't change other lines!
  
  *y = 4;
  
  assert_eq!(*x, 5);

  println!("Success!");
}