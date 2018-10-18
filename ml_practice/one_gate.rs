/* 
    This file includes stuff to test a one gate transformer.
*/

// Define complexity of neural network calculations.
pub enum TransformComplexity {
    Simple,
    Random,
}

// A transformer that does some stuff with inputs
fn transform_multiply_gate(i: f32, j: f32) -> f32 {
    // Return the multiplied value
    return i * j;
}

// A function to increase output via random local search.
fn process_random(i: f32, j: f32) -> f32 {
    // Get the initial output of the two inputs
    let mut output: f32 = i * j;

    // Set a multiplier
    let multiplier: f32 = 0.00001;

    let mut i_best: f32 = i;
    let mut j_best: f32 = j;

    // Try 100 times
    for x in 0..100 { 

        // Get a random number
        let i_atpt = i_best + (get_random_number() * multiplier);

        // Get a random number
        let j_atpt = j_best + (get_random_number() * multiplier);

        // Aggregate the two numbers onto the output
        let output_atpt = transform_multiply_gate(i_atpt, j_atpt);

        // Check if the new output is greater than the current value
        if output_atpt > output {
            // If so, switch to a new output.
            i_best = i_atpt;
            j_best = j_atpt;
            output = output_atpt;
        }
    }

    return output; 
}

// Fake random number generator.
fn get_random_number() -> f32 {
    return 2.0;
}

// A function to do simple 1 gate manipulation
fn process_simple(i: f32, j: f32) -> f32 {
    // Put the inputs through the transform gate
    let output: f32 = transform_multiply_gate(i, j);

    // Return the output
    return output;
} 

// A public interface to execute transformations.
pub fn transform_example(tc: TransformComplexity) {
    // Set input values
    // NOTE: integers vs floats are distinct.
    //       cannot implicitly identify as a float
    let input_1: f32 = 12.0;
    let input_2: f32 = 3.0;

    // Pass inputs into the gates
    let output: f32;
    match tc {
        TransformComplexity::Simple => { output = process_simple(input_1, input_2); }
        TransformComplexity::Random => { output = process_random(input_1, input_2); }
    }

    // Print out the result
    println!("input: {}, {}", input_1, input_2);
    println!("output: {}", output);
}
