use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn err_handling_ex() {
    println!("\n====== Error handling example ======");
    //panic!("crash and burn!!")

    println!(
        "Current directory: {}",
        std::env::current_dir().unwrap().display()
    );

    // Simple panic
    println!("******Simple panic when file does not exists");
    panic_on_file_open();

    //More panic actions
    println!("******More actions on panic");
    more_action_when_painc_happens();

    //Propagating errors
    println!("******Propagating error from function");
    let username_verbose = read_username_from_file_verbose();
    println!("Username verbose read: {}", username_verbose.unwrap());
    let username_shortcut = read_username_from_file_shortcut();
    println!("Username shortcut read: {}", username_shortcut.unwrap());
    let username_simple = read_username_from_file_simplest();
    println!("Username simple read: {}", username_simple.unwrap());

    //Question(?) operator for error handling
    println!("*****(?) operator for error handling");
    let char=last_char_of_first_line("jane doe");
    println!("last char is {:?}", char);
}

fn panic_on_file_open() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    println!("File {:?}", greeting_file)
}

fn more_action_when_painc_happens() {
    let file_to_open = "hello1.txt";
    let greeting_file_result = File::open(file_to_open);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_to_open) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    println!("Creting file if doesnt exists {:?}", greeting_file);

    //Shortcuts for Panic on Error: unwrap and expect
    //let greeting_file = File::open("hello.txt").unwrap();

    //let greeting_file = File::open("hello2.txt")
    //.expect("hello.txt should be included in this project");
}

fn read_username_from_file_verbose() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_shortcut() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_simplest() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
