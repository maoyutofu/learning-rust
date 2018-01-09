// rust程序大部分由一系列的语句构成
fn main() {
    // 语句
    // 语句
    // 语句
    // rust有多种语句。最普遍的类型有两种：一种是变量绑定；另一种是表达式带上分号
    // 变量绑定
    let x = 5;
    // 表达式
    x;
    x + 1;
    15;
    // 代码块也是表达式，所以它在赋值操作中可以充当右值(r-values)
    // 代码块最后一条表达式将赋给左值（l-values）
    // 如果代码最后一条表达式有分号，那么返回值将变成()
    // 代码最后一条语句是实际执行的最后一条语句，并不一定是最行一行代码

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 这里表达式将给y
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了表达式。于是将`()`给了z
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}