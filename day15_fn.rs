// 函数使用fn关键字来声明
// 函数的参数需要标注类型 如果有返回值，必须在箭头->之后特别指出

// 函数的最后表达式将作为返回值 或者 在函数内部使用return语句来提前返回值

// 和C/C++ 不一样，Rust的函数定义位置是没有限制的
fn main() {
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

// 函数不返回值，实际上返回的是一个单元类型 `()`
fn fizzbuuz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuuz(n);
    }
}