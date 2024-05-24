// Solve it in two ways
// DON'T let `println!` work
pub fn main() {
    let result: u8 = sum(255, 1);
    // asse
    println!("result___{}", result);
    never_return();

    println!("Failed!");
}


fn never_return() -> ! {
   panic!();
    // Implement this function, don't modify the fn signatures
}


// integers:
// u8 - 255 
fn sum(x: u8, y: u8) -> u8 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    #[test]
    #[should_panic]
    fn test_sum_should_panic() {
        sum(255, 1);
    }
}

