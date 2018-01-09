// type语句可以给一个已存在的类型起一个新的名字
// 类型必须要有CamelCase驼峰方式的名称，否则会产生一个警告 对规则例外的是基本类型如usize，f32等


// `NanoSecond`是`u64`的新名字
type NanoSecond = u64;
type Inch = u64;

// 使用一个属性来忽略警告
#[allow(none_camel_case_types)]
type u64_t = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // 注意类型的别名没有提供任何的类型安全，因为它不是一个新的类型
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}