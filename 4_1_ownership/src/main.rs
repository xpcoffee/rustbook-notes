fn main() {
    string_literal();
    mutate_string();
    copy_pointer();
    clone_data();
    stack_copy();
}

fn string_literal() {
    let s1 = "hello"; // string literal; lives on the stack; cannot be mutated
    let s2 = ", world!"; // string literal; cannot be mutated
    println!("{}{}", s1, s2);
}

fn mutate_string() {
    let mut s1 = String::from("hello"); // lives on the heap; can be mutated
    s1.push_str(", world!");

    println!("{}", s1);
}

fn copy_pointer() {
    let s1 = String::from("hello!");
    let s2 = s1;

    // would fail - pointer ownership has moved to s2
    // println!("s1 {}", s1);

    // works
    println!("s2 {}", s2);
}

fn clone_data() {
    let s1 = String::from("hello!");
    let s2 = s1.clone();
    println!("s1 {}, s2 {}", s1, s2);
}

fn stack_copy() {
    let x = 5; // x is known at compile time
    let y = x; // so x can cheaply & reliably be copied on the stack for y

    println!("x {} y {}", x, y); // valid, pointer to 5 has not been moved
                                 // the above uses the Copy trait (more on that later)
}

// Copy and Drop traits are mutually exclusive
// Copy means "store in stack"
// Drop means "release from heap", which doesn't make sense with the above
