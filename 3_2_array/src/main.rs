use std::io;

fn main() {
    let array = [8; 3];

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    let index: usize = user_input
        .trim()
        .parse()
        .expect("Unable to convert user input into index");
    let element = array[index];

    println!("Array {}", element);
}
