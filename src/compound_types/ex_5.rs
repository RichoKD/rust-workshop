// Initial problem

// Fill the blank
// fn main() {
//     let s = String::from("I like dogs");
//     // Allocate new memory and store the modified string there
//     let s1 = s.__("dogs", "cats");

//     assert_eq!(s1, "I like cats");

//     println!("Success!");
// }

// Fill the blank
pub fn main() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    // The replace String method is use to replace a string slice from a String
    // The first parameter is a string slice which represent the string slice you wish to replace
    //The second parameter is a string slice which represent the new string slice to replace the old one.
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success!");
}
