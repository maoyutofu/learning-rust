// if - else 分支判断鹤其它语言类似
// 与很多语言不同的是，Rust语言中的布尔判断条件不用小括号
// 每个判断条件后连着一个代码块
// if - else条件选择是一个表达式，并且所以分支都必须返回相同的类型

fn main() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            10 * n
        } else {
            println!(", and is big number , reduce by two");

            n / 2
        };

        println!("{} -> {}", n, big_n);
}