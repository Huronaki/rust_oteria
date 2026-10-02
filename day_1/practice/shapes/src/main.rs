struct Rect {
    w: f64,
    h: f64,
}

impl Rect {
    fn new(w: f64, h: f64) -> Rect {
        Rect { w, h }
    }

    fn area(&self) -> f64 {
        self.w * self.h
    }

    fn is_square(&self) -> bool {
        self.w == self.h
    }

    fn grow(&mut self, by: f64) {
        self.w += by;
        self.h += by;
    }
}

fn main() {
    let mut rect = Rect::new(4.0, 4.0);

    println!("area = {}\nis_square = {}", rect.area(), rect.is_square());

    rect.grow(1.0);

    println!(
        "after grow: {} x {} (area = {})",
        rect.w,
        rect.h,
        rect.area()
    );
}
