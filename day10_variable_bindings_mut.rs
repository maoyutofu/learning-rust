// 变量默认情况是不可变的，加上mut修饰后变量就是可变的

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // 错误，没有mut修饰，不可变
    // _immutable_binding += 1;
}