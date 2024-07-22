struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }
}

struct NoShape {} // unit struct
impl NoShape {
    fn area(&self) -> i32 {
        return 0;
    }
}

fn main() {
    let rect = Rect {
        width: 20,
        height: 10,
    };
    let area = rect.area();
    let perimeter = rect.perimeter();
    println!("perimeter = {}", perimeter);
    println!("area = {}", area);

    let no_shape = NoShape{};
    println!("noshape area = {}", no_shape.area());
}
