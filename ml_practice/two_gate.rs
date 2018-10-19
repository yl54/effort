/* 
    This file includes stuff to test a two gate transformer.
*/

// A list of options to process
pub enum TransformComplexity {
    AnalyticGradient,
}

// A transformer that does stuff with inputs
fn transform_multiply_gate(i: f32, j: f32) -> f32 {
    return i * j;
}

// A transformer that does other stuff with inputs
fn transform_subtraction_gate(i: f32, j: f32) -> f32 {
    return i - j;
}

/*
    A compound transformer

    x -- |  |
    y -- |  | --- |   |
                  |   | -- q
    z ----------- |   |
*/
fn transform_compound(i: f32, j: f32, k: f32) -> f32 {
    // Get i - j = m
    let subt: f32 = i - j;

    // Get m * k = q
    let mult: f32 = subt * k;

    // Return output
    let output: f32 = mult;
    return output;
}

// A function to calculate the analytic gradient.
fn process_analytic_gradient(i: f32, j: f32, k: f32) -> f32 {
    // Set the step amount
    let step: f32 = 0.01;

    // Set the initial variables
    let mut i_next: f32 = i;
    let mut j_next: f32 = j;
    let mut k_next: f32 = k;
    let mut output: f32 = 0.0;

    // Get i, j derivatives
    let i_deriv: f32 = 1.0;
    let j_deriv: f32 = -1.0;

    // Try a for loop
    for x in 0..10 {
        // Get both outcomes in separate variables
        let subt_1: f32 = transform_subtraction_gate(i_next, j_next);
        let mult_2: f32 = transform_multiply_gate(subt_1, k_next);

        // Get the derivatives of i and j
        let deriv_f_k: f32 = subt_1; // want
        let deriv_f_subt_1: f32 = k_next;

        // Get the derivatives of m and k
        let deriv_f_i = i_deriv * deriv_f_subt_1; // want
        let deriv_f_j = j_deriv * deriv_f_subt_1; // want

        // Apply force to each input
        i_next = i_next + (step * deriv_f_i);
        j_next = j_next + (step * deriv_f_j);
        k_next = k_next + (step * deriv_f_k);

        output = transform_compound(i_next, j_next, k_next);
        println!("output: {}", output);
    }

    // Return the output
    return output;
}

// An example function to calculate stuff
pub fn transform_example(tc: TransformComplexity) {
    // Set up initial inputs
    let input_1: f32 = -3.0;
    let input_2: f32 = 2.0;
    let input_3: f32 = 4.0;

    // Select processing method based off of choice
    let output: f32;
    match tc {
        TransformComplexity::AnalyticGradient => { output = process_analytic_gradient(input_1, input_2, input_3); }
    }

    // Print out inputs and output
    println!("input_1: {}", input_1);
    println!("input_2: {}", input_2);
    println!("output: {}", output);
}