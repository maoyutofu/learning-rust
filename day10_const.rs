// rust有两种常量，可以在任意作用域声明，包括全局作用域。这两种常量都要显示的标注
// const: 不可改变的值（常用类型）
// static: 在'static生命周期内可能发生的改变的值

// 有个特例，就是“string”原始类型。可以给它直接赋一个不可改变的static变量。
// 是因为它的类型标记：&'static str包含了生命周期'static
// 其它的引用类型都必须特别注明从而拥有'static生命周期

// 在所有的作用域外声明全局变量
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在main函数中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！ 不能修改一个`const`常量
    // THRESHOLD = 5;
}