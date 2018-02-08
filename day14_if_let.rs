// 在一些例子中 match 使用起来并不优雅，比如
// 将`optional`定为`Option<i32>`类型

// let optional = Some(7);

// match optional {
//     Some(i) => {
//         println!("This is a really long string and `{:?}`", i);
//     },
//     _ = {},
//     // 这里_成了必须内容，因为`match`需要覆盖全部情况。难道不觉得冗余吗
// };

// 使用if let对这样的用法就简洁很多，并且允许指明特定的各种不同的失败可选项内容
fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let`结构解读：若`let`将`number`结构成`Some(i)`，则执行语句块`{}`
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败的情形可以用else
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供一个可以改变的失败条件
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }
}