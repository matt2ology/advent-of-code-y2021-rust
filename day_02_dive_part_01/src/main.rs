use std::fs::read_to_string; // read_to_string is a function that reads a file into a string

/**
# Day 2: Dive

Takes a file with a list of lines that contain a character and an integer value.
The character can be 'f' for forward, 'u' for up, or 'd' for down.
The integer value is the number of units to move in the direction specified by the character.
The starting position is (0, 0) and the final position is the product of the horizontal and vertical position.

## Example

```md
f 5
u 3
d 9
```

1. The first line moves forward 5 units, so the position is now (5, 0).
2. The second line moves up 3 units, so the position is now (5, -3).
3. The third line moves down 9 units, so the position is now (5, 6).
4. The final position is (5, 6), so the answer is 30.
 */
fn main() {
    let puzzle_input_file: &str = "./../../input_files/day_02_dive/input.txt";
    // Tuple of (horizontal_position, vertical_position)
    let mut horizontal_vertical_position: (u32, u32) = (0, 0);

    // Parse file line by line and update position based on character in line (f, u, d) and integer value
    read_to_string(puzzle_input_file)
        .expect("Unable to open file")
        .lines()
        .for_each(|line: &str| {
            // Print line "forward 5"
            println!("{} ", line);
            // Parse line "forward 5" into tuple ('f', 5)
            let line_tuple: (char, u32) = (
                line.chars().next().unwrap(),
                line.split_at(line.find(' ').unwrap())
                    .1
                    .trim()
                    .parse()
                    .unwrap(),
            );

            // Print processed line tuple ('f', 5)
            print!("{:?} ", line_tuple);

            // Update horizontal_vertical_position based on line_tuple
            match line_tuple.0 {
                // forward horizontal movement add value to horizontal position
                'f' => horizontal_vertical_position.0 += line_tuple.1,
                // upward vertical movement subtract value to vertical position
                'u' => horizontal_vertical_position.1 -= line_tuple.1,
                // downward vertical movement add value to vertical position
                'd' => horizontal_vertical_position.1 += line_tuple.1,
                _ => (), // Do nothing
            }

            // Print current horizontal_vertical_position
            println!("{:?}", horizontal_vertical_position);
        }); // End of for_each

    // Multiply horizontal and vertical position to get final answer
    println!(
        "Answer: {}",
        horizontal_vertical_position.0 * horizontal_vertical_position.1
    );
}
