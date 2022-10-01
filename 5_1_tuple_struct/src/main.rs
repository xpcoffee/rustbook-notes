struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let some_color = Color(1, 2, 3);
    let some_point = Point(4, 5, 6); // different type, even though data-fields seem similar

    println!("The third value of black {}", some_color.2);
    println!("The first value of origin {}", some_point.0);
}
