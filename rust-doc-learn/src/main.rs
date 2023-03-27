mod common_prog_concepts;
use crate::common_prog_concepts::consts;
use crate::common_prog_concepts::data_types;
use crate::common_prog_concepts::functions;
use crate::common_prog_concepts::immutability;
use crate::common_prog_concepts::shadow;
use crate::common_prog_concepts::control_flows;

mod ownership;
use crate::ownership::ownership_in_rust;
use crate::ownership::ref_and_borrow;

fn main() {
    println!("Hello, Rustaceans!");

    //Common Programming Concepts
    immutability::immutability();
    consts::constants();
    shadow::shadowing();
    data_types::data_types();
    functions::function_example();
    control_flows::control_flows();

    //Understanding Ownership 
    ownership_in_rust::ownerships();
    ref_and_borrow::reference_and_borrows();
    
}
