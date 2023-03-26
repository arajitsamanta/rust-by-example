pub fn data_types() {
    println!("\n====== Data types example ======");
    println!("******Scalar Types");
    scalars_types();

    println!("******Numeric operations");
    numeric_operations();

    println!("******Booleans");
    bools();

    println!("******Characters");
    chars();

    println!("******Compound Types");
    compound_types();
}

fn scalars_types() {
    //Integers
    let i8_max: i8 = (isize::pow(2, 7) - 1).try_into().unwrap();
    let i8_min: i8 = (-(isize::pow(2, 7))).try_into().unwrap();
    println!("8 bit signed integer: max {i8_max}, min {i8_min}");

    let i16_max: i16 = (isize::pow(2, 15) - 1).try_into().unwrap();
    let i16_min: i16 = (-(isize::pow(2, 15))).try_into().unwrap();
    println!("16 bit signed integer: max {i16_max}, min {i16_min}");

    let i32_max: i32 = (isize::pow(2, 31) - 1).try_into().unwrap();
    let i32_min: i32 = (-(isize::pow(2, 31))).try_into().unwrap();
    println!("32 bit signed integer: max {i32_max}, min {i32_min}");

    //let _i64_max=((2i64.pow(63) as i128)-1) as i64;
    //let i64_min:i64=(-(isize::pow(2,63))).try_into().unwrap();
    //println!("64 bit signed integer: max {i64_max}, min {i64_min}");

    //Literals
    let literal_int = 98_90;
    let literal_hex = 0xff;
    let literal_octal = 0o77;
    let literal_binary = 0b1111_0000;
    let literal_byte = b'A';
    println!("Literals: Integer {literal_int}, Hexdecimal {literal_hex}, Octal {literal_octal} Binary {literal_binary} Byte {literal_byte}");

    //Floating point
    let x = 2.07; // f64
    let y: f32 = 3.01; // f32
    println!("FLoating types: f64 {x} f32 {y} ");
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {sum}, difference: {difference} product: {product} quotient: {quotient} truncated: {truncated} remainder: {remainder}")
}

fn bools() {
    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("true: {t} false: {f}")
}

fn chars() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Simple Char: {c}, explicit type conversion: {z}, emoji char {heart_eyed_cat}")
}

fn compound_types(){

    //Tupple
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    let _tup_unit= (); // No printable???

    println!("Tuple index0: {five_hundred}, index1: {six_point_four}, index2: {one}");

    //Arrays
    let _a = [1, 2, 3, 4, 5];

    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // Array with types element are is i32
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    //Array with repeated values
    let a = [3; 5];
    println!("Array : {:?}", a);

}