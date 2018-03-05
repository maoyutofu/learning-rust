// 闭包（closure）在rust中也称为lambda 是一类捕获封闭环境的函数。
// 例如 一个可以捕获x变量的闭包如下：
// |val| val + x

// 她们的语法和能力使它们在临时（on the fly）使用相当方便。调用一个闭包和调用一个函数完全相同
// 然而，输入和返回类型两者都可以自动推导，且输入变量名必须指明

// 其他特点包括：
// 使用||替代()将输入变量括起来
// 区块定界符号{} 对于单条表达式是可以选的，其他情况必须加上
// 有能力不过到外部的环境变量
fn main() {
    // 过通闭包和函数实现增量
    fn function (i: i32) -> i32 { i + 1}

    // 闭包是匿名的，这里我们将它们绑定到引用
    // 类型标注和函数的一样，不过类型标注和使用 {} 来围住代码都是可选的
    // 这些匿名函数（nameless function）赋值给合适命名的变量
    let closure_annotated = |i: i32| -> i32 { i +1 };
    let closure_inferred = |i | i+ 1;

    let i = 1;

    // 调用函数和闭包
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 没有参数的闭包 返回一个 i32 类型
    // 返回类型是自动推导的
    let one = || 1;
    println!("closure returning one: {}", one());
}