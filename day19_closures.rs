struct Cacher<T> where T: Fn(i32) -> i32 {
	calculation: T,
	value: Option<i32>,
}

impl<T> Cacher<T> where T: Fn(i32) -> i32 {
	fn new(calculation: T) -> Cacher<T> {
		Cacher {
			calculation,
			value: None,
		}
	}

	fn value(&mut self, arg: i32) -> i32 {
		match self.value {
			Some(v) => v,
			None => {
				let v = (self.calculation)(arg);
				self.value = Some(v);
				v
			},
		}
	}
}

fn main() {
	// 闭包。将函数给变量，在需要的时候使用就可以了
	// 这个是传入一个x 返回 x
	let example_closuer = |x| x;

	// let s = example_closuer(String::from("hello"));
	let n = example_closuer(5);

	//  闭包 将函数的实现交给后来实现
	let mut espensive_result = Cacher::new(|num| {
		println!("calculating slowly...");
		num
	});
}