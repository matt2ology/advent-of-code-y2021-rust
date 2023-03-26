use std::fs::read_to_string;

fn main() {
    // &str references a string literal. A string literal is a sequence of characters
    // surrounded by double quotes. String literals are immutable.
    // https://doc.rust-lang.org/reference/types.html#string-literals
    let file_input_path: &str = "./../../input_files/day_01/input.txt";
    // Populate vector with values from file
    let mut depth_values: Vec<u32> = Vec::new();
    // Depth increment sum counter
    let mut depth_counter: u32 = 0;
    // Temporary variable to hold value of previous value in sliding window
    let mut previous_value: u32 = 0;

    // Populate vector with values from file
    read_to_string(file_input_path)
        .expect("Unable to open file")
        .lines()
        .for_each(|line: &str| {
            // parse string to u32 and add to vector.
            depth_values.push(line.parse::<u32>().unwrap());
        });

    // -2 because we are using a sliding window of 3 values in vector (i, i+1, i+2)
    for i in 0..depth_values.len() - 2 {
        // sliding window of 3 values in vector (i, i+1, i+2)
        let sliding_window_3: Vec<u32> = depth_values[i..i + 3].to_vec();
        // Add values in sliding window together
        let sliding_window_3_sum: u32 = sliding_window_3.iter().sum();

        // if let no previous value, set previous value to first value in sliding window
        if previous_value == 0 {
            println!("{} (N/A - no previous sum)", previous_value);
        } else if sliding_window_3_sum > previous_value {
            println!("{} > {} : Increased", sliding_window_3_sum, previous_value);
            increment_depth_counter(&mut depth_counter);
        } else if sliding_window_3_sum < previous_value {
            println!("{} < {} : Decreased", sliding_window_3_sum, previous_value);
        } else {
            println!(
                "{} = {} : Same - no change",
                sliding_window_3_sum, previous_value
            );
        }

        // Update previous value to current value
        previous_value = sliding_window_3_sum;
    }

    // Print depth counter
    println!(
        "There are {} measurements that are larger than the previous measurement",
        depth_counter
    );
}

/**
   Increase depth counter
*/
fn increment_depth_counter(depth_counter: &mut u32) {
    *depth_counter += 1;
}
