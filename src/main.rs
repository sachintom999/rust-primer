enum Direction {
    North,
    South,
    East,
    West,
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn main() {
    let my_direction = Direction::North;
    move_around(my_direction);
    let cicle = Shape::Circle(5.0);
    let area = calc_area(cicle);
    println!("area = {}", area);
}

fn move_around(direction: Direction) {

    // some logic
}

fn calc_area(shape: Shape) -> f64 {
    return 0.0;
}
