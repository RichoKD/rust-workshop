pub fn main() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("ex 3: success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
