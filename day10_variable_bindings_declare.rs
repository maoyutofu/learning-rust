// rust可以先声明变量绑定，后来才初始化值。但这种情况很少用。因为可能导致使用未初始化的变量

fn main() {
    // 声明一个变量绑定
    let a_binding;

     {
         let x =2;
         // 初始化一个绑定
         a_binding = x * x;
     }

     println!("a binding: {}", a_binding);

     let another_binding;

     // 报错，使用了初始化的绑定
    //  println!("another binding: {}", another_binding);

     another_binding = 1;
     println!("another binding: {}", another_binding);
}