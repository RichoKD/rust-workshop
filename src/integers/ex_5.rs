// Fix errors and panics to make it work
pub fn int_ex_5() {
    let v1: u16 = 251_u8 as u16  + 8_u8 as u16;
    let v2 = i8::checked_add(251_u8 as i8, 8_u8 as i8).unwrap();
    println!("log int_ex_5: {},{}",v1,v2);
 }