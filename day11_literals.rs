// 数字字面量可以加上类型标记作为后缀来标注类型。
// 比如字面量 42为i32类型，可以写成42i32
// 如果未加上后缀的类型会视情况而定。比如编译器将整型定位i32，浮点定位f64类型

fn main() {
    // 有后缀的字面量
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量 编辑器视情况而定
    let i = 1;

    let f = 1.0;

    // `size_of_val`返回变量大小，以字节byte为单位
    use std::mem::{size_of_val};
    println!("size of x in bytes: {}", size_of_val(&x));
    println!("size of y in bytes: {}", size_of_val(&y));
    println!("size of z in bytes: {}", size_of_val(&z));
    println!("size of i in bytes: {}", size_of_val(&i));
    println!("size of f in bytes: {}", size_of_val(&f));
}