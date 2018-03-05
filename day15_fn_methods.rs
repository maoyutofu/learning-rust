struct Point {
    x: f64,
    y: f64,
}

// 实现的代码块，所有的Point方法都在这里给出
impl Point {
    // 这是一个静态方法 （static method）
    // 静态方法不需要通过实例来调用
    // 这类方法一般用作构造器（constructor）
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另外一个静态方法，带有两个参数
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这里是实例方法 （instance method）
    // &self 是 self: &Self 的语法糖（sugar） 其中Self是所调用对象的类型
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用者对象是可变的
    // `&mut self` 为 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // first,second 离开作用域后释放
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    // 实例方法通过点运算符来调用
    // 注意第一个参数 &self 是隐式传递的 比如
    // rectangle.perimeter() === Rectangle::perimeter(&rectangle)
    print!("Rectangle perimeter: {}", rectangle.perimeter());
    print!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 报错 rectangle 是不可变的，但这方法需要一个可变的对象
    // rectangle.translate(1.0, 1.0);
    square.translate(1.0, 0.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // 报错，签名的destory 调用 消费了 pair
    // pair.destory();
}