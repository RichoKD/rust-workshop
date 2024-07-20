// Don't use clone ,use copy instead
pub fn main() {
    let x = (1, 2, (), &"hello".to_string()); // To be able to copy this type, make all it's individual value copyable by the compiler
    // let x = (1, 2, (), "hello"); // Approach 1, use string slices; they're stored on the stack and can easily be copied by the compiler.
    let x = (1, 2, (), &"hello".to_string()); // Approach 2, use references (borrowing) for every value stored on the heap, to avoid the compiler implicitly moving your data.
    let y = x;
    println!("{:?}, {:?}", x, y);
}