//  类型推导引擎是相当智能的。
// 不仅仅在初始化期间分析右值的类型，还会通过分析变量在后面是怎么使的来推导该变量的类型。

// 一个类型推导的高级例子

fn main() {
    // 借助类型标注 编辑器知道`elem`是u8类型
    let elem = 5u8;

    // 创建一个空的vector
    let mut vec = Vec::new();
    // 此时编译器并未知道`vec`的确切类型，它只知道`vec`是一个含有某种类型的vector Vec<_>

    // 将`elem`插入vector
    vec.push(elem);
    // 现在编译器就知道`vec`是一个含有`u8`类型的vector/Vec<u8>
    println!("{:?}", vec);
}