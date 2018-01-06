// 变量绑定有一个作用域。并且限定在一个代码块中存活。{}

fn main() {
    // 此绑定存在于main函数中
    let long_lived_binding = 1;

    // 一个代码块，比main有更小的作用域
    {
        // 此绑定只存在于本代码块
        let shor_lived_binding = 2;

        println!("inner short: {}", shor_lived_binding);

        // 此绑定*隐藏*了外面的绑定
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // 报错 不存在此作用域内
    // println!("outer short! {}", short_lived_binding);

    println!("outter long: {}", long_lived_binding);

    // 此绑定隐藏了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);
}