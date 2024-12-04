use std::fs::File;
use std::io::{self, BufRead};

pub fn check_if_row_is_valid(row: &Vec<i32>) -> (bool, i32) {
    // returns a tuple of (is_safe, and the bad index if the row is not safe)
    let mut is_safe = 1;
    let mut is_increasing = 0; // 0 is unset, 1 is increasing, -1 is decreasing
    let mut bad_index = -1;
    for i in 0..row.len() - 1 {
        // distance check
        let abs_diff = (row[i] - row[i + 1]).abs();
        if abs_diff > 3 || abs_diff < 1 {
            is_safe = 0;
            bad_index = i as i32;
            break;
        }

        // number is safe if all increasing or all decreasing and only by gaps of 1 or 2
        if row[i] < row[i + 1] {
            if is_increasing == 0 {
                is_increasing = 1;
            } else if is_increasing == -1 {
                is_safe = 0;
                bad_index = i as i32;
                break;
            }
        } else if row[i] > row[i + 1] {
            if is_increasing == 0 {
                is_increasing = -1;
            } else if is_increasing == 1 {
                is_safe = 0;
                bad_index = i as i32;
                break;
            }
        } else {
            is_safe = 0;
            bad_index = i as i32;
            break;
        }
    }
    return (is_safe == 1, bad_index);
}

fn main() -> io::Result<()> {
    // Define the path to your input file
    let input_path = "input.txt";

    // Open the file
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    // Read lines from the file
    let mut num_safe = 0;
    for line in reader.lines() {
        // Get the line as a string
        let line = line?;

        // split on whitespace, and parse the two numbers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // check if the row is valid. if not, retry with each index removed to see if it fixes the
        let mut global_is_safe = 0;
        let (is_safe, _) = check_if_row_is_valid(&numbers);
        if !is_safe {
            for i in 0..numbers.len() {
                let mut row = numbers.clone();
                row.remove(i);
                let (is_safe, _) = check_if_row_is_valid(&row);
                if is_safe {
                    global_is_safe = 1;
                    break;
                }
            }
        } else {
            global_is_safe = 1;
        }
        num_safe += global_is_safe;

        // now let's just check the "first bad index" solution to see where it misses
        let (new_is_safe, bad_index) = check_if_row_is_valid(&numbers);
        if !new_is_safe {
            //println!("Row {:?} is not safe at index {}", numbers, bad_index);
            let mut row2 = numbers.clone();
            row2.remove(bad_index as usize);
            let (inner_is_safe, _) = check_if_row_is_valid(&row2);
            if global_is_safe > 0 && !inner_is_safe {
                println!(
                    "Row {:?} is safe but the first bad index ({:?}) solution says it's not",
                    numbers, bad_index
                );
            }
        }
    }

    // Print the result
    println!("Number of safe numbers: {}", num_safe);
    Ok(())
}
