#[allow(dead_code)]
pub fn symbol() {
    // Addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);

    // Multiplication
    println!("2 * 2 = {}", 2u32 * 2);

    // Bitwise OR
    println!("0011 OR 0101 = {:04b}", 0b0011u32 | 0b0101);

    // Bitwise XOR
    println!("0011 XOR 0101 = {:04b}", 0b0011u32 ^ 0b0101);

    // Bitwise AND
    println!("0011 AND 0101 = {:04b}", 0b0011u32 & 0b0101);

    // Bitwise Shift Left
    println!("1 << 5 = {}", 1u32 << 5);

    // Bitwise Shift Right
    println!("0x80 >> 2 = 0x{:x}", 0x80u32 >> 2);
}


pub fn methods(x: i32, y: i32) {
    
}