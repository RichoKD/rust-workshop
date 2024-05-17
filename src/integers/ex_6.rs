// Modify `assert!` to make it work
pub fn int_ex_6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v != 1579);

    println!("log success from int_ex_6 {:?} !", assert!(v != 1579));
}
