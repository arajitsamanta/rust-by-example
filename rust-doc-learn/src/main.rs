mod common_prog_concepts;
use crate::common_prog_concepts::consts;
use crate::common_prog_concepts::data_types;
use crate::common_prog_concepts::functions;
use crate::common_prog_concepts::immutability;
use crate::common_prog_concepts::shadow;
use crate::common_prog_concepts::control_flows;

fn main() {
    println!("Hello, Rustaceans!");
    immutability::immutability();
    consts::constants();
    shadow::shadowing();
    data_types::data_types();
    functions::function_example();
    control_flows::control_flows();
}
