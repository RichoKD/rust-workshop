pub fn main() {
  let x = 5;
  // Fill the blank
  // I added the & operator because p is borrowing the value of x
  let p = &x;

  println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
  // the output is 0x16fa3ac84 because of `{:p}` if it was just {}, the output will be 5
  println!("the memory address of x is {}", p);
}