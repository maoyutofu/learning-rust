// 单行注释
/* 块注释 */

/// 对接下来的项生成帮助文档
/// 对封闭项生成帮助文档
fn main() {
	//! 对封闭项生成帮助文档
	// 这里是行注释
	// 这里的内容编译器是不会读取的
	/*
	* 块注释。用于注释临时代码非常有效
	 */
	 //! 对封闭项生成帮助文档
	let x = 5 + 5;
	print!("Is `x` 10 or 100? x = {}", x);
}