// 🌟🌟 We can use a pattern with let to destructure a tuple to separate variables.
// Tips: you can use Shadowing or Mutability

// Fix the error below with least amount of modification
pub fn destructure() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success from exercise 6!: {}", x);
}