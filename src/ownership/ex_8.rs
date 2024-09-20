pub fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.clone();

    // Modify this line only, don't use `_s`
    println!("{:?}", t);
}
