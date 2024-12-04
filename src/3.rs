use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let input_path = "input.txt";

    // Open the file
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);

    // Read in the file content
    let mut input = String::new();
    reader.read_to_string(&mut input)?;

    println!("File content:\n{}", input);

    let mut total = 0;
    let mut mul_enabled = true; // `mul` starts enabled by default

    for part in input.split("mul(").skip(1) {
        // Check for `do()` or `don't()` before processing

        //if !mul_enabled {
        //    println!("Skipping `mul` at part (disabled): {}", part);
        //    continue; // Skip if `mul` is disabled
        //}

        if let Some(end_pos) = part.find(')')
            && mul_enabled
        {
            let inner = &part[..end_pos];
            let mut nums = inner.split(',');

            if let (Some(first), Some(second)) = (nums.next(), nums.next()) {
                if nums.next().is_none() {
                    if let (Ok(f), Ok(s)) =
                        (first.trim().parse::<i32>(), second.trim().parse::<i32>())
                    {
                        total += f * s;
                        println!("Valid: mul({}, {}) -> Product = {}", f, s, f * s);
                    }
                }
            }
        }
        if part.contains("don't()") {
            mul_enabled = false;
            println!("Disabling `mul` at part: {}", part);
            continue;
        } else if part.contains("do()") {
            mul_enabled = true;
            println!("Enabling `mul` at part: {}", part);
        }
    }

    println!("Final Total: {}", total);

    Ok(())
}
