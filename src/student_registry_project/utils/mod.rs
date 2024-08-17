use std::io;
use std::io::Write; // for flush

pub fn convert_to_string(x: &str) -> String {
    x.to_string()
}

pub fn convert_to_u32(input: &str) -> u32 {
    input
        .trim()
        .parse::<u32>()
        .expect("Invalid input. Please enter a number.")
}

pub fn input_u32(prompt: &str) -> u32 {
    input_any(prompt)
        .trim()
        .parse::<u32>()
        .expect("Invalid input. Please enter a number.")
}

pub fn input_u8(prompt: &str) -> u8 {
    input_any(prompt)
        .trim()
        .parse::<u8>()
        .expect("Invalid input. Please enter a number.")
}

pub fn input_f32(prompt: &str) -> f32 {
    input_any(prompt)
        .trim()
        .parse::<f32>()
        .expect("Invalid input. Please enter a number.")
    
}

pub fn input_any(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
        
}
// }