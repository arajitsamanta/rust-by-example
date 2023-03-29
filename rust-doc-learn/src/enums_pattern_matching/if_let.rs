pub fn if_let_ex() {
    println!("\n====== If let ======");

    //Match expresssion
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Using match - The maximum is configured to be {}", max),
        _ => (),
    }

    //Rewirte match using if-let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("Using if let - The maximum is configured to be {}", max);
    }
}
