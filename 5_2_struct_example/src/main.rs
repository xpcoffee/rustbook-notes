fn main() {
    let width = 30;
    let height = 50;

    println!(
        "Individual {} x {} = {}",
        width,
        height,
        area(width, height)
    );

    println!(
        "Tuple {} x {} = {}",
        width,
        height,
        area_with_tuple((width, height))
    );

    let rect = Rectangle { width, height };
    println!("Struct | {:?} | area = {}", rect, area_with_struct(&rect));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
