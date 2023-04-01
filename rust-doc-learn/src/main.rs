mod common_prog_concepts;
mod structures;
use crate::common_prog_concepts::consts;
use crate::common_prog_concepts::control_flows;
use crate::common_prog_concepts::data_types;
use crate::common_prog_concepts::functions;
use crate::common_prog_concepts::immutability;
use crate::common_prog_concepts::shadow;

mod ownership;
use crate::ownership::ownership_in_rust;
use crate::ownership::ref_and_borrow;
use crate::ownership::slices;
use crate::structures::struct_impl;
use crate::structures::structures_ex;

mod enums_pattern_matching;
use crate::enums_pattern_matching::enums;
use crate::enums_pattern_matching::if_let;
use crate::enums_pattern_matching::match_control_flow;

mod collections;
use crate::collections::utf8_string;
use crate::collections::vector;
use crate::collections::hash_map;


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
    slices::slices_example();

    //Structures
    structures_ex::structure_example();
    struct_impl::struct_with_method_impl();

    //Enums
    enums::enum_ex();
    match_control_flow::match_ex();
    if_let::if_let_ex();

    //Collections
    vector::vector_ex();
    utf8_string::utf8_string_ex();
    hash_map::hash_map_ex();
}
