// rust在基本类型之间没用提供隐式类型转换（强制类型转换）
// 可以使用as关键字进行显示类型转换

fn main() {
    let decimal = 65.4321_f32;
    // 报错，不能隐式类型转换
    // let integer: u8 = decimal;

    // 显示类型转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当任意整数值转成无符号类型（unsigned类型）T时
    // 将会加上或减去 std::T::MAX + 1 直到符合新的类型

    // 1000原本就符合u16类型
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 在计算机底层会截取数字的低8位
    // 而高位数字会被抛掉
    println!("1000 as a u8 is: {}", 1000 as u8);
    // -1 + 256 = 255
    println!(" -1 as a u8 is: {}", (-1i8) as u8);

    // 对正数来说 上面的类型转换操作和取模效果一样
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当将整数值转换成有符号类型(signed类型)时 同样要现将（二进制）数值
    // 转成相应的无符号类型。如i32和u32 i16和u16
    // 然后再求此值的补码 如果数值的最高位是1则数值是负数
    
    // 除非值本来就已经符合所要转的类型
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128 再求数字128的8位二进制补码得到
    println!(" 128 as i8 is: {}", 128 as i8);

    // 1000 as u8 -> 232
    println!("1000 as a i8 is: {}", 1000 as i8);
    // 232的补码是-24
    println!(" 232 as a i8 is: {}", 232 as i8);
}