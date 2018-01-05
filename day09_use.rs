#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 明确的`use`各个名称使他们直接可用不需要手动加上作用域
    use Status::{Poor, Rich};

    // 自动的`use`内部的各个名称
    use Work::*;

    // 等价于`Status::Poor`
    let status = Poor;
    // 等价于`Status::POor`
    let work = Civilian;

    match status {
        // 注意这里少了作用域，因为上面用`use`显示的导入了
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}