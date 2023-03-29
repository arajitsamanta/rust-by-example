#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn match_ex() {
    println!("\n====== Match Control Flow ======");
    let penny = Coin::Penny;
    println!("Penny: {:?}", value_in_cents(penny));
    println!("Nickel: {:?}", Coin::Nickel);
    println!("Dime: {:?}", Coin::Dime);
    println!(
        "Quarter in: '{:?}' is {:?}",
        UsState::Alaska,
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    println!(
        "Quarter in: '{:?}' is {:?}",
        UsState::Alabama,
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
    println!(
        "Plus one: {:?}, Plus None: {:?}",
        plus_one(Some(5)),
        plus_one(None)
    );

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //other => move_player(other),
        //_ => reroll(),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {
    println!("add_fancy_hat")
}
fn remove_fancy_hat() {
    println!("remove_fancy_hat")
}
fn _move_player(num_spaces: u8) {
    println!("move_player {}", num_spaces)
}
fn _reroll() {
    println!("reroll")
}
