fn main() {
	// 分配在堆中
	// 特性：为了安全，资源只保留一份
	let s1 = String::from("hello");

	// s1到s2发生了move操作
	// 在使用s1时就会报错
	// let s2 = s1;

	// 	error[E0382]: use of moved value: `s1`
	//  --> day16_ownership.rs:8:17
	//   |
	// 7 |     let s2 = s1;
	//   |         -- value moved here
	// 8 |     println!("{}", s1);
	//   |                    ^^ value used here after move

	// 深拷贝 显示的调用clone
	// 如果是基础类型不需要clone  自动实现了拷贝
	let s2 = s1.clone();

	println!("{}", s1);
	println!("{}", s2);

	let s = String::from("world");

	takes_ownership(s);
	// - value moved here

	// println!("{}", s);
	// ^ value used here after move

	let x = 5;

	makes_copy(x);

	println!("{}", x);

	let s3 = gives_ownership();

	println!("{}", s3);

	let s5 = String::from("developer");

	let s6 = takes_and_gives_back(s5);

	println!("{}", s6);
}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}

fn gives_ownership() -> String {
	let some_string = String::from("hi");
	some_string
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}