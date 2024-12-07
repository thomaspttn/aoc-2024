use std::fs::File;
use std::io::{self, BufRead};

pub fn put_bad_rule_in_order(
    rule: Vec<i32>,
    hashmap: &std::collections::HashMap<i32, Vec<i32>>,
) -> i32 {
    // the goal of this function is to use the hashmap we have about the correct ordering to put a
    // rule into the correct order. what we can do here is: when we find a value that has broken a
    // rule, we can look up the correct order in the hashmap and move the value to the correct
    // place in the list. we can then check if the rule is valid again.
    // my only concern right now: will we definitely be able to put it into the right spot? let's
    // see i guess
    println!("Rule: {:?}", rule);

    let mut result = rule.clone();

    // start with a while valid
    let mut valid = false;
    while !valid {
        valid = true;
        let mut seen = std::collections::HashSet::new();
        let mut i = 0; // Manual iteration for better control
        while i < result.len() {
            let this_value = result[i].clone(); // Clone the value to avoid borrowing `result`

            if let Some(values) = hashmap.get(&this_value) {
                let mut this_value_is_valid = true;

                for value in values {
                    if seen.contains(value) {
                        valid = false;
                        this_value_is_valid = false;
                        break;
                    }
                }

                if !this_value_is_valid {
                    println!("i: {} Rule: {:?}", i, result);

                    let mut current_index = i;
                    while !this_value_is_valid {
                        println!("Current index: {}", current_index);
                        let new_index = current_index.saturating_sub(1);

                        // Build a "hypothetical seen" set
                        let hypothetical_seen: std::collections::HashSet<_> =
                            result.iter().take(new_index).cloned().collect();

                        // Check if the value is valid at the new index
                        if !values.iter().any(|v| hypothetical_seen.contains(v)) {
                            println!("Value is valid at index: {}", new_index);

                            // Move the value to the new index
                            let value = result.remove(i);
                            result.insert(new_index, value);

                            this_value_is_valid = true;
                        } else {
                            current_index = new_index;
                        }
                    }
                }
            }
            seen.insert(this_value); // Add the value to the `seen` set
            i += 1; // Increment index
        }
    }
    println!("Result: {:?}", result);

    // return the midpoint value
    result[result.len() / 2]
}

fn main() -> io::Result<()> {
    // Define the path to your input file
    let input_path = "input.txt";

    // Open the file
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    // read lines until we get a blank line
    // each line is two numbers delimited by a |
    // we want to create a hashmap, mapping the first number to a list containing the second number
    // if a first number occurrs again, we want to append the second number to the list
    // after the blank line, we get the rules we want to evaluate the hashmap with
    // these are just numbers delimited by commas

    let mut hashmap = std::collections::HashMap::new();
    let mut rules = Vec::new();
    let mut blank_line = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            blank_line = true;
            continue;
        }
        if !blank_line {
            let mut parts = line.split("|");
            let key = parts.next().unwrap().parse::<i32>().unwrap();
            let value = parts.next().unwrap().parse::<i32>().unwrap();
            let entry = hashmap.entry(key).or_insert(Vec::new());
            entry.push(value);
        } else {
            // parse the rule line into a vector of i32
            let rule: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
            rules.push(rule);
        }
    }

    println!("{:?}", hashmap);
    println!("{:?}", rules);

    let mut midpoint_sum = 0;
    let mut bad_midpoint_sum = 0;

    // now let's evaluate the rules one by one
    for rule in &rules {
        // empty set of values we've seen so far (i32)
        let mut seen = std::collections::HashSet::new();
        let mut valid = true;
        for item in rule {
            // check if item is in our hashmap. if so: every value it's linked to should **not**
            // have come before it. so for each value, check if it's in the seen set. if so, the
            // rule is invalid
            if hashmap.contains_key(&item) {
                let values = hashmap.get(&item).unwrap();
                for value in values {
                    if seen.contains(value) {
                        valid = false;
                        break;
                    }
                }
            }
            // add the item to the seen set
            seen.insert(item);
        }
        println!("Valid: {}", valid);

        // get the value at the midpoint of the rule and add it to the sum if the rule is valid
        if valid {
            let midpoint = rule[rule.len() / 2];
            midpoint_sum += midpoint;
        } else {
            let bad_midpoint = put_bad_rule_in_order(rule.to_vec(), &hashmap);
            bad_midpoint_sum += bad_midpoint;
        }
    }

    println!("Midpoint sum: {}", midpoint_sum);
    println!("Bad Midpoint sum: {}", bad_midpoint_sum);

    Ok(())
}
