pub fn vector_ex() {
    println!("\n====== Vector example ======");

    println!("******Create vector");
    create_vector();

    println!("******Update vector");
    update_vector();

    println!("******Access vector");
    acccess_vector();

    println!("******Iterate vector");
    iterate_vector();

    println!("******Multi type vector");
    multiple_types_vector();
}

fn create_vector() {
    let v: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v);

    let mut v_init = vec![1, 2, 3];
    v_init.sort();
    println!("Vector with initial value: {:?}", v_init);
}

fn update_vector() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("Vector after updating: {:?}", v);
}

fn acccess_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn iterate_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
}

fn multiple_types_vector() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Multiple type vector {:?}", row);
}
