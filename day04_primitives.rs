fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0; //常规声明
    let an_inter = 5i32; //后缀声明
    
    // 自动推断
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // 可变类型`i32`
    let mut mutable = 12;
    // 报错 变量类型不可变
    // mutable = true;
}