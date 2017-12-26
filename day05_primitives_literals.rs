// 在rust中数字可以加上前缀 0x、0o、0b分别表示十六进制、八进制、二进制数
// 为了改善数字的可读性，可以在数字类型直接加上下划线（_），比如：1_000等于1000
// 0.000_001 等于 0.000001

fn main() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("Not true is {}", !true);

    // 位运算符
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性
    println!("One million is written as {}", 1_000_000u32);
}