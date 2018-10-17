/* 
    This file includes stuff to test a one gate transformer.
*/

// Define complexity of neural network calculations.
pub enum TransformComplexity {
    Simple,
}

// A transformer that does some stuff with inputs
fn transform_multiply_gate(i: i16, j: i16) -> i16 {
    // Return the multiplied value
    return i * j;
}

// A function to do 1 gate manipulation
fn transform_full_simple(i: i16, j: i16) -> i16 {
    // Put the inputs through the transform gate
    let output: i16 = transform_multiply_gate(i, j);

    // Return the output
    return output;
} 

// An example function to have its own inputs
pub fn transform_example(tc: TransformComplexity) {
    // Set input values
    let input_1: i16 = 12;
    let input_2: i16 = 3;

    // Pass inputs into the gates
    let output: i16;
    match tc {
        TransformComplexity::Simple => {
            output = transform_full_simple(input_1, input_2);
        }
    }

    // Print out the result
    println!("input: {}, {}", input_1, input_2);
    println!("output: {}", output);
}
