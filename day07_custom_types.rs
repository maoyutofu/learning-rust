// rust中主要通过 struct 和 enum 两个关键字来创建自定义类型
// 通过const 和 static 关键字来创建常量


// 结构体。structure简写struct  有3种类型
// 元组结构体，就是根据元组来命名
// C语言风格结构体 c_struct
// 单元结构体  不带字段  在范型中很有用

// 单元结构体
struct Nil;

// 元组结构体
 struct Pair(i32, f32);

 // 带有两个字段的结构体
 struct Point {
     x: f32,
     y: f32,
 }

 // 结构体可以是另外一个结构体的字段
 #[allow(dead_code)]
 struct Rectangle {
     p1: Point,
     p2: Point,
 }

 fn main() {
     // 实例化结构体`Point`
     let point: Point = Point{ x: 0.3, y: 0.4 };

     // 访问`point`的字段
     println!("point coordinates: ({}, {})", point.x, point.y);

     // 使用`let`绑定来解构 point
     let Point {x: my_x, y: my_y} = point;

     let _rectangle = Rectangle {
         p1: Point { x: my_x, y: my_y },
         p2: point,
     };

     // 实例化一个单元结构体
     let _nil = Nil;

     // 实例化一个元组结构体
     let pair = Pair(1, 0.1);

     // 访问元组结构体的字段
     println!("pair contains {:?} and {:?}", pair.0, pair.1);

     // 解构一个元组结构体
     let Pair(integer, decimal) = pair;
     println!("pair contains {:?} and {:?}", integer, decimal);
}