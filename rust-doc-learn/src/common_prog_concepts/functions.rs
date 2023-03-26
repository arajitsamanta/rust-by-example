pub fn function_example() {
    println!("\n====== Functions example ======");
    function_with_parameter(14);

    print_labeled_measurement(5, 'h');

    expressions();

    println!("Function with return value: {} - {}", five(), plus_one(5))
}

fn function_with_parameter(x: i32) {
    println!("The value of function parameter x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("Expression evaluation - the value of y is: {y}");
}

fn five() -> i32 {
    return 5;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
