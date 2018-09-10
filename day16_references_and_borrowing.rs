fn main() {
	let s1 = String::from("hello");

	let len = calculate_length(&s1);

	println!("The lengtgh of '{}' is {}.", s1, len);

	let mut s = String::from("world");
	change(&mut s);
	// 在一个作用域里可变的借用，只能有一次，不能多次mut 避免数据竞争




	// 悬垂引用
	// 数据清除了 把没有用的引用 返回了回去
	let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
	s.len()
}

// 需要加上 可变的 mut 不然不能修改
// use `&mut String` here to make mutable
fn change(some_string: &mut String) {
	some_string.push_str("!");
	// cannot borrow as mutable
}

// fn dangle() -> &String {
// 	let s = String::from("hello");
// 	// 离开作用域 销毁了 返回了无效的引用
// 	&s
// }
// 解决办法 移交所有权
fn dangle() -> String {
	let s = String::from("hello");
	// 离开作用域 销毁了 返回了无效的引用
	s
}