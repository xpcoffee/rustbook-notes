fn main() {
    for_loops_with_label();
    while_loop();
    for_in();
    for_in_range();
}

fn for_loops_with_label() {
    let mut counter = 0;
    let result = 'counting_up: loop {
        println!("count {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining {remaining}");
            if remaining == 0 {
                break;
            }

            remaining -= 1;
            if counter == 2 {
                break 'counting_up counter * 2;
            }
        }

        counter += 1;
    };

    println!("The result is {result}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!")
}

fn for_in() {
    let a = [1, 2, 3, 3, 4, 5, 6];

    for element in a {
        println!("element {element}");
    }
}

fn for_in_range() {
    // excl of end [1, 4)
    for element in (1..4).rev() {
        println!("{element}!");
    }
    println!("LIFTOFF!");
}
