#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Enum: {:?}", self);
    }
}

pub fn enum_ex() {
    println!("\n====== Enums ======");

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    //Option enum
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!(
        "Built in enum: {:?} - {:?} - {:?}",
        some_number, some_char, absent_number
    )
}
