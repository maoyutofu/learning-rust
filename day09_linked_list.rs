// enum的一个常见的用法就是创建链表 linked-list

use List::*;

enum List {
    // Cons 元组结构体，包含一个元素和一个执行下一个节点的指针
    Cons(u32, Box<List>),
    // Nil 末节点，表明链表结束
    Nil,
}

impl List {
    // 创建一个空的列表
    fn new() -> List {
        // `Nil` 为 `List`类型
        Nil
    }

    // 处理一个列表，得到一个头部带上一个新元素的同样类型的列表并返回此值
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // 返回列表的长度
    fn len(&self) -> u32 {
        // `self`必须匹配，因为这个方法的行为取决于`self`的变化类型
        // `self`为`&List`类型，`*self`为`List`类型，一个具体的`T`类型的匹配
        // 要参考引用`&T`的匹配
        match *self {
            // 不能得到 tail 的所有权，因为`self`是借用的
            // 而是得到一个tail引用
            Cons(_, ref tail) => 1 + tail.len(),
            // 基本情形，空列表的长度为0
            Nil => 0
        }
    }

    // 将列表以字符串（堆分配）的形式返回
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 创建一个空的链表
    let mut list = List::new();

    // 追加元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}