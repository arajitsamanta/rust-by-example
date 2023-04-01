pub fn utf8_string_ex() {
    println!("\n====== String as collection example ======");

    println!("******Create string");
    create_string();

    println!("******Update string");
    update_string();

    println!("******Concatenation string");
    concat_string();

    println!("******Slicing string");
    slice_string();

    println!("******Iterate string");
    iterate_string();
}

fn create_string() {
    let s = String::new();

    println!("Empty creation: {}", s);

    let data = "initial contents";

    let s = data.to_string();

    println!("String with initial content: {}", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    println!("String with initial content using literal {}", s);

    println!("Greeting in multiple language");
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");
}

fn update_string() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1} and s2 is {s2}");

    //push a single character
    let mut s = String::from("lo");
    s.push('l');
    println!("String single char push {}", s);
}

fn concat_string() {
    let s1 = String::from("hello ");
    let s2 = String::from("world");

    let s3 = s1 + &s2;
    println!("Concatenated string using {s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Multiple string concatenation {s}")
}

fn slice_string() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("String slice: {}", s)
}

fn iterate_string() {
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
