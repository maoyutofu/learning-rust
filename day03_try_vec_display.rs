// 对于一个结构体来说，要对各个元素实现fmt::Display可能会很麻烦。问题在于每个write!都要生成一个fmt::Result。
// 彻底实现，需要处理所有的结果。为此Rust提供了try!宏

// 在write!上使用try!类似这样：
// 对 `write!`进行try，观察是否会出错。若发生错误，返回相应的错误
// try!(write!(f, "{}", value))

// example
use std::fmt;

// 定义一个包含不同`Vec`的结构提 `List`
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 对`self`解引用，并通过解构创建一个指向`vec`的引用
        let List(ref vec) = *self;
        try!(write!(f, "["));
        // 对`vec`进行迭代，`v`是每次迭代的值，`count`是计数器
        for (count, v) in vec.iter().enumerate() {
            // 调用`write!`前对每个元素（第一个元素除外）加上逗号
            // 使用`try!`，在出错的情况返回
            if count != 0 {try!(write!(f, ", "));}
            try!(write!(f, "{}:{}", count, v));
        }
        // 加上配对的中括号，并返回一个fmt::Result值
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}