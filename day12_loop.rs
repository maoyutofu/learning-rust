// loop关键字实现无限循环
// break跳出  continue跳过剩余部分，开始新的一轮循环

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}