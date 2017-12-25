use std::fmt;

struct Structure(i32);

// fmt::Display没用不能输出Vec<T> 泛型容器的实现。泛型的情况下要用fmt::Debug
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Minmax(i64, i64);

impl fmt::Display for Minmax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct A {
    real: f64,
    imag: f64,
}

impl fmt::Display for A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.real, self.imag)
    }
}

impl fmt::Debug for A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.real, self.imag)
    }
}

fn main() {
    println!("{}", Structure(6));
    println!("{}", Minmax(7, 8));
    println!("{}", Point2D{x: 3.3, y: 7.2});
    println!("{:?}", Point2D{x: 3.3, y: 7.2});
    println!("{}", A{real: 3.3, imag: 7.2});
    println!("{:?}", A{real: 3.3, imag: 7.2});
}
