mod immutability;
mod consts;
mod shadow;
mod data_types;

fn main() {
    println!("Hello, Rustaceans!");
    immutability::immutability();
    consts::constants();
    shadow::shadowing();
    data_types::data_types();
}
