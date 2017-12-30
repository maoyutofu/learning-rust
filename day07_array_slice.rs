// 数组是一组同类型数据的集合，并存储在连续的内存中。数组使用中括号[]来创建。
// 数组的大小在编译期间就已经确定，数组的类型标记为[T; size]

// slice 中文是“切片”的意思，与数组类似，但slice类型的大小在编译期间是不确定的
// slice是agiel双字对象（two-word object），第一个字是一个指向数据的指针
// 第二个字是切片的长度
// slice可以用来借用数组的一部分。slice的类型标记为&[T]

use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len())
}

fn main() {
    // 固定大小的数组 （类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所以元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 索引从0开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在堆中分配
    println!("array occupies: {}", mem::size_of_val(&xs));

    // 数组可以自动化的借用成为`slice`
    println!("borrow the whole array as a alice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&xs[1 .. 4]);

    // 越界的索引会引发panic
    // println!("{}", xs[5]);
}