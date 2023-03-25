use std::fs; // for reading file contents from file system (fs)

/**
Rust solution to Advent of Code 2021 - Day 1: Sonar Sweep
 */
fn main() {
    let file_input_path = "./../../input_files/day_01/input.txt";
    // open file for reading
    let file_contents = fs::read_to_string(file_input_path).expect("Unable to open file");
    // vector to store values from lines in file
    let mut values: Vec<i32> = Vec::new();
    // start at 0 depth counter (no depth changes yet) - we will increase this as we go through the file
    let mut depth_counter: i32 = 0;

    /* loop through lines in file */
    file_contents.lines().for_each(|line| {
        values.push(line.parse::<i32>().unwrap());
    });

    /*
    loop through vector values and compare to next value in
    vector to see if it increased, decreased or stayed the same (no change)
    - We use `-1` because we are comparing to next value in vector
    - 0..values.len() loops through a vector (0..5 = 0, 1, 2, 3, 4)
     */
    for i in 0..values.len() - 1 {
        let next_sweep: i32 = values[i + 1];
        println!("Current depth: {}", depth_counter);
        if next_sweep > values[i] {
            println!("{} > {} : Increased", next_sweep, values[i]);
            increase_depth_counter(&mut depth_counter);
        } else if next_sweep < values[i] {
            println!("{} < {} : Decreased", next_sweep, values[i]);
        } else {
            println!("{} = {} : Same - no change", next_sweep, values[i]);
        }
    }

    println!(
        "There are {} measurements that are larger than the previous measurement",
        depth_counter
    );
}

/**
 Increase depth counter by 1
* @param `depth_counter` - reference to depth counter
* @return void

## Notes on Rust syntax

- [`&mut` is a mutable reference to a value](https://doc.rust-lang.org/reference/expressions/reference-expr.html#mutable-references)
- [The `*` is a dereference operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-dereference-operator)
- [It's like a pointer in C or C++](https://www.geeksforgeeks.org/cpp-pointers/)
r)
    - [It's a reference to the value that the pointer points to](https://stackoverflow.com/questions/3730019/what-is-the-difference-between-a-pointer-variable-and-a-pointe */
fn increase_depth_counter(depth_counter: &mut i32) {
    *depth_counter += 1;
}
