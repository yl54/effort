/* 
    This file includes stuff to test a one gate transformer.
*/

// Define complexity of neural network calculations.
pub enum TransformComplexity {
    Simple,
    Random,
    NumericalGradient,
    AnalyticGradient,
}

// A transformer that does some stuff with inputs
fn transform_multiply_gate(i: f32, j: f32) -> f32 {
    // Return the multiplied value
    return i * j;
}

// A function to do simple 1 gate manipulation
fn process_simple(i: f32, j: f32) -> f32 {
    // Put the inputs through the transform gate
    let output: f32 = transform_multiply_gate(i, j);

    // Return the output
    return output;
} 

// A function to increase output via random local search.
// The goal is to continuously tweak the input values to increase the output.
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

// A function to do a numerical gradient analysis.
fn process_numerical_gradient(i: f32, j: f32) -> f32 {
    // Get the initial output
    let mut output = transform_multiply_gate(i, j);

    // Set a tweak amount
    let tweak: f32 = 0.001;

    // Set a step amount.
    let step: f32 = -0.01;

    // Set initial variables.
    let mut i_next: f32 = i;
    let mut j_next: f32 = j;
    let mut output_next: f32 = 0.0;

    // Try a for loop
    for x in 0..1000 {
        // Calculate the derivative for i
        let i_deriv: f32 = get_derivative(i_next, j_next, tweak);
        // println!("i_deriv: {}", i_deriv); 

        // Calculate the derivative for j
        let j_deriv: f32 = get_derivative(j_next, i_next, tweak);
        // println!("j_deriv: {}", j_deriv);

        // Apply this to every input as a gradient.
        // x = x + x derivative * step
        i_next = i_next + (i_deriv * step);
        j_next = j_next + (j_deriv * step);

        output_next = transform_multiply_gate(i_next, j_next);

        println!("i_deriv: {}", i_deriv);
        println!("j_deriv: {}", j_deriv);
        println!("i_next: {}", i_next);
        println!("j_next: {}", j_next);
        println!("output_next: {}", output_next);
        println!("");
    }

    return output_next;
}

// A function to do analytic gradient analysis
// For this particular transformation, the derivative happens to be the other value
// Direct expression = fewer computations
// In the real world, you compute the analytic gradient to figure things out so that
//   your performance does not suck.
fn process_analytic_gradient(i: f32, j: f32) -> f32 {
    // Get the initial output
    let output: f32 = i * j;

    // Set an step amount 
    let step: f32 = 0.01;

    // Set initial values to iterate over
    let mut i_next: f32 = i;
    let mut j_next: f32 = j;
    let mut output_next: f32 = output;
    
    // Try a for loop
    for x in 0..100 {
        // Get the next i input 
        i_next = i_next + (j_next * step);

        // Get the next j input
        j_next = j_next + (i_next * step);

        // Get the next output
        output_next = transform_multiply_gate(i_next, j_next);
    }

    // Return the last output that came out
    return output_next;
}

// Fake random number generator.
fn get_random_number() -> f32 {
    return 2.0;
}

/*
    Derivative generator for:

    (main + tweak) * adj + (main) * adj 
    ------------------------------------
                   tweak

*/
fn get_derivative(main: f32, adj: f32, tweak: f32) -> f32 {
    // Get changed value
    let changed = (main + tweak) * adj;

    // Get anchored value
    let anchored = main * adj;

    // Do changed - anchored
    let diff = changed - anchored;

    // divide by tweak
    let deriv = diff / tweak;

    return deriv;
}

// A public interface to execute transformations.
pub fn transform_example(tc: TransformComplexity) {
    // Set input values
    // NOTE: integers vs floats are distinct.
    //       cannot implicitly identify as a float
    let input_1: f32 = 2.0;
    let input_2: f32 = -3.0;

    // Pass inputs into the gates
    let output: f32;
    match tc {
        TransformComplexity::Simple => { output = process_simple(input_1, input_2); }
        TransformComplexity::Random => { output = process_random(input_1, input_2); }
        TransformComplexity::NumericalGradient => { output = process_numerical_gradient(input_1, input_2); }
        TransformComplexity::AnalyticGradient => { output = process_analytic_gradient(input_1, input_2); }
    }

    // Print out the result
    println!("input: {}, {}", input_1, input_2);
    println!("output: {}", output);
}
