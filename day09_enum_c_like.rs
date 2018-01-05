// enum也可以像C语言枚举那样使用

#![allow(dead_code)]

// 拥有隐式的辨别值，从0开始计数
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显示的辨别值
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main()  {
    use Number::{ Zero, One, Two };
    // `enum` 可以转换成整型
    println!("zero is {}", Zero as i32);
    println!("one is {}", One as i32);
    println!("two is {}", Two as i32);

    use Color::*;
    println!("roses ar #{:06x}", Red as i32);
    println!("violets are #{:06x}", Blue as i32);
}