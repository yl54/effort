/* 
    This file is practicing basic ml math.
    Rust is just to practice the language.
*/

#![allow(unused_mut)]
#![allow(dead_code)] 
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unreachable_patterns)]
#![allow(unused_assignments)]

pub mod one_gate;

// Main function
fn main() {
    // Try a simple solution.
    let tc: one_gate::TransformComplexity = one_gate::TransformComplexity::Simple;
    one_gate::transform_example(tc);
}
