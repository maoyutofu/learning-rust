// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
// fn large(a: &str, b: &str) -> &str {
// 	if a.len() > b.len() {
// 		a
// 	}
// 	b
// }

// 添加生命周期 让a b  返回值 活的一样长
// 让返回值跟参数活的一样长
// 函数默认会加生命周期 只是多个参数 不知道用哪个
// 如果只有一个参数 就不需要显示指定
fn large<'a>(a: &'a str, b: &'a str) -> &'a str {
	if a.len() > b.len() {
		a
	} else {
		b
	}
}

// 让name的生命周期跟struct一样长
struct Person<'a> {
	name: &'a str,
}

impl <'a> Person<'a> {

}

fn main() {
	// let r;
	// {
	// 	let x = 5;
	// 	r = &x;
	// }

	// 报错 r找到x的引用 但是x已失去生命周期
	// println!("r: {}", r);


	let str1 = "123";
	let str2 = "12345";
	let str3 = large(&str1, &str2);
	println!("{}", str1);
	println!("{}", str2);
	println!("{}", str3);

	// let str2 = large(&"123", &"12345");
	// println!("{}", str2);
}