fn normal_function() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}

// Parameters

fn Parameters() {
    this_function(5);
}

fn this_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn MultipleParameters() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Statements and Expressions

fn Statements() {
    let y = 6;
}

fn Expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

// Functions with Return Values

fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}