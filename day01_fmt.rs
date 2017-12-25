use std::fmt;

fn main() {
	// 如果不加后缀31自动转为 I32类型
	println!("{} days", 31);

	println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

	println!("{subject} {verb} {object}", 
		object = "the lazy dog",
		verb = "the quick brown fox",
		subject="jumps over");

	println!("{} of {:b} people know binary, the other half don't", 1, 2);
	// 不足位数，前面会填充空格
	println!("{number:>width$}", number=1, width=6);
	// 不足，补0
	println!("{number:0width$}", number=1, width=6 );

	#[allow(dead_code)]
	struct Structure(i32);
	impl fmt::Display for Structure {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{}", self.0)
		}
	}
	print!("This struct `{}` won't print...", Structure(3));
}