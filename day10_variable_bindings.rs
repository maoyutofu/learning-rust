// rust通过静态类型确保类型安全。变量绑定可以在声明变量时标注类型。不过在多数
// 情况下，编译器能够从字面内容推导出变量的类型，大大减少标注类型的负担

// 使用let绑定操作，可以将值绑定到变量中

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将`an_integer`复制到`copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用变量绑定产生警告，可在变量名加上下划线来消除这些警告
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
}