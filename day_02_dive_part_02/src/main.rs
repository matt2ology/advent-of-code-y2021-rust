use std::fs::read_to_string; // read_to_string is a function that reads a file into a string

// Struct to hold submarine data - horizontal position, depth and aim
struct Submarine {
    horizontal_position: u64,
    depth: u64,
    aim: u64,
}

// Struct to hold movement data - movement type and movement value
struct Movement {
    movement_type: char,
    movement_value: u64,
}

fn main() {
    let day_02_puzzle_input_file: &str = "./../../input_files/day_02_dive/input.txt";

    let mut submarine: Submarine = Submarine {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    read_to_string(day_02_puzzle_input_file)
        .expect("Unable to open file")
        .lines()
        .for_each(|line: &str| {
            // Read file line by line and process each line

            let line_input_tuple = Movement {
                // Create Movement struct from line
                movement_type: line.chars().next().unwrap(), // Get first character of string
                movement_value: line
                    .split_at(line.find(' ').unwrap()) // Split string at space
                    .1 // Get string after space character (movement value)
                    .trim() // Remove whitespace from string before and after
                    .parse() // Parse string to u32 integer type (Result type)
                    .unwrap(), // Unwrap u32 integer type from Result type
            };

            match line_input_tuple.movement_type {
                'f' => {
                    submarine.horizontal_position += line_input_tuple.movement_value;
                    submarine.depth += submarine.aim * line_input_tuple.movement_value;
                },
                'u' => submarine.depth -= line_input_tuple.movement_value,
                'd' => {
                    submarine.depth += line_input_tuple.movement_value;
                    submarine.aim += line_input_tuple.movement_value;
                },
                _ => (), // Do nothing
            }

            println!(
                "Submarine aim {} * horizontal {} = depth {}",
                submarine.aim, submarine.horizontal_position, submarine.depth
            )
        });

    println!("Submarine depth: {}", submarine.depth);
}
