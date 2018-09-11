use std::fs::File;
use std::io;

fn oepn_file() ->Result<File, io::Error> {
	let f = File::open("./test.txt")?;
	Ok(f)
}

fn main() {
	// panic!("不可以恢复的错误，相当于exit，退出程序");

	// let f = File::open("./test.txt");
	// match f {
	// 	Ok(str2) => {
	// 		println!("{:?}", str2)
	// 	}
	// 	Err(err) => {
	// 		println!("{:?}", err)
	// 	}
	// }

	// unwrap什么都不处理  打不开 直接崩溃  panic
	// let f = File::open("./test.txt").unwrap();
	// expect 自定义错误信息
	// let f = File::open("./test.txt").expect("找不到文件");
	// 传递错误  等于抛出错误到上级
	// let f = File::open("./test.txt")?;
	let a = oepn_file();
	// if let Ok(f) = a {
	// 	println!("{:?}", f);
	// }
	if let Err(f) = a {
		println!("{:?}", f);
	}
}