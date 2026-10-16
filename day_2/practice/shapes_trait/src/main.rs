trait Shape {
    fn name(&self) -> &'static str;
    fn area(&self) -> f64;
}

struct Circle {
    r: f64,
}

struct Square {
    side: f64,
}

struct Rect {
    w: f64,
    h: f64,
}

impl Shape for Circle {
    fn name(&self) -> &'static str {
        "circle"
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

impl Shape for Square {
    fn name(&self) -> &'static str {
        "square"
    }

    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Rect {
    fn name(&self) -> &'static str {
        "rect"
    }

    fn area(&self) -> f64 {
        self.w * self.h
    }
}

fn describe(shape: &impl Shape) {
    println!("{} -> area {:.2}", shape.name(), shape.area());
}

fn main() {
    let circle = Circle { r: 2.0 };
    let square = Square { side: 3.0 };
    let rect = Rect { w: 2.0, h: 4.0 };

    describe(&circle);
    describe(&square);
    describe(&rect);
}
