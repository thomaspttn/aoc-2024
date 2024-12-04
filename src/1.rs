use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    // Define the path to your input file
    let input_path = "input.txt";

    // Open the file
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    // let left column be a vec of numbers
    let mut left_column: Vec<i32> = Vec::new();

    // store right column as a hash map
    // key: number, value: count
    let mut map: HashMap<i32, i32> = HashMap::new();

    // Read lines from the file
    for line in reader.lines() {
        // Get the line as a string
        let line = line?;

        // split on whitespace, and parse the two numbers
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // add left number to column
        left_column.push(numbers[0]);

        // add right number to map if not in it with count of 1, else incr the count
        let count = map.entry(numbers[1]).or_insert(0);
        *count += 1;
    }

    // sort the columns
    left_column.sort();

    // sum up left count times num occurrences of num in the right map
    let mut total = 0;
    for i in 0..left_column.len() {
        let num = left_column[i];

        // get the count of the number or zero
        let count = map.get(&num).unwrap_or(&0);

        // print out num and count
        println!("Num: {}, Count: {}", num, count);
        total += count * num
    }

    // Print the result
    println!("Total: {}", total);
    Ok(())
}
