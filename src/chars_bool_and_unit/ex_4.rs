// Make it work
pub fn main() {
    let f = true;
    let t = true && false;
    assert_ne!(t, f);

    println!("success from bool 2: Success!");
}
