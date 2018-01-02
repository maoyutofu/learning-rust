// enum 关键字允许创建一个代表数个可能变量的数据的类型
// 在struct中任何合法的变量在enum同样是合法的

// 隐藏未使用代码警告的属性
#![allow(dead_code)]

// 创建一个`enum`（枚举）来划分人的类型。注意命名鹤类型的信息是如何一起的
// 明确规定变量的
// `Enginner != Scientist`鹤`Height(i32) != Weight(i32)`
// 每个都不相同且相互独立
enum Person {
    // 一个`enum`可能是个`unit-like`（类单元结构体）
    Enginner,
    Scientist,
    // 或者像是一个元组结构体
    Height(i32),
    Weight(i32),
    // 或像是一个普通的结构体
    Info { name:String, height: i32 }
}

// 此函数将一个`Person` enum 作为参数，无返回值
fn inspect(p: Person) {
    // `enum`的使用必须覆盖所以的情形（无可辩驳），所以使用`match`
    // 以分支方式覆盖所以的类型
    match p {
        Person::Enginner => println!("Is enginner!"),
        Person::Scientist => println!("Is scientist!"),
        
        // 从`enum`内部解构`i`
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),

        // 将`Info`解构`name`和`height`
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    let amira = Person::Weight(10);

    // `to_owned()`从一个字符串slice创建一个具有所有权的`String`
    let dave = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let rohan = Person::Enginner;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}