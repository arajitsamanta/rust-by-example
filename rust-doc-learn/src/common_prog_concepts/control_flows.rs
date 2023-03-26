pub fn control_flows() {
    println!("\n====== Control flows example ======");
    println!("******if's");
    if_cond();

    println!("******loops");
    loops();

    println!("******while loop");
    whiles();

    println!("******For loop");
    for_loop()
}

fn if_cond(){
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Using if in let statement
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

}
fn loops() {
    //loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");


}

fn whiles(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //Looping through collection's
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop(){
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!("for loop with reverse");
    for number in (1..4).rev() {
        println!("{number}");
    }
   
}