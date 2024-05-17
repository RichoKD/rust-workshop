// Unit Exercise

// Make it work, don't modify `implicitly_ret_unit` !
pub fn main() {
    let _v: () = (); // unit

    let v = (2, 3);
    // println!("v here: {:?} ", &v);

    // implicitly_ret_unit();
    // let mut return_implicit = ();
    // return_implicit = implicitly_ret_unit();
    // return_implicit = v;

    assert_eq!(_v, implicitly_ret_unit());

    println!("success from unit: Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()"); // returns unit - ()
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}