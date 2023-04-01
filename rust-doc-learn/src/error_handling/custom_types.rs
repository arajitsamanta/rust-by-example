pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn custom_types_on_err() {

    println!("\n====== Using custom type ======");
    let guess = "32";
    loop {
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        println!("Guess value {}", guess.value());
        break;

        // match guess.cmp(&secret_number) {
        // --snip--
        //}
    }
    
}
