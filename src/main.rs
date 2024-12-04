use std::fs::File;
use std::io::{self, BufRead};

pub fn dfs(
    rows: &Vec<Vec<char>>,
    x: isize,
    y: isize,
    word: &Vec<char>,
    index: usize,
    dx: isize,
    dy: isize,
) -> bool {
    if index == word.len() {
        return true; // Found the word
    }

    // Check bounds
    if x < 0 || y < 0 || x >= rows.len() as isize || y >= rows[0].len() as isize {
        return false;
    }

    // Check current character
    if rows[x as usize][y as usize] != word[index] {
        return false;
    }

    // Continue searching in the same direction
    dfs(rows, x + dx, y + dy, word, index + 1, dx, dy)
}

pub fn sliding_window(rows: Vec<Vec<char>>, match_pattern: Vec<Vec<char>>, count: &mut usize) {
    println!("Sliding window");

    // grid is 3x3, so need to slide that along the rows and columns
    for i in 0..rows.len() - 2 {
        for j in 0..rows[0].len() - 2 {
            let mut found = true;
            for k in 0..3 {
                for l in 0..3 {
                    if match_pattern[k][l] != '.' && rows[i + k][j + l] != match_pattern[k][l] {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }
            if found {
                *count += 1;
            }
        }
    }
}

pub fn find_x_mas(rows: Vec<Vec<char>>) -> usize {
    // the goal here: look for MAS in the shape of an X. this only has 4 ways of happening so we
    // can slide a 2d window over the grid and check for the pattern for each of the 4.

    let match_pattern_1 = vec![
        vec!['M', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'S'],
    ];
    let match_pattern_2 = vec![
        vec!['S', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'M'],
    ];
    let match_pattern_3 = vec![
        vec!['S', '.', 'M'],
        vec!['.', 'A', '.'],
        vec!['S', '.', 'M'],
    ];
    let match_pattern_4 = vec![
        vec!['M', '.', 'M'],
        vec!['.', 'A', '.'],
        vec!['S', '.', 'S'],
    ];

    let mut count = 0;
    sliding_window(rows.clone(), match_pattern_1, &mut count);
    sliding_window(rows.clone(), match_pattern_2, &mut count);
    sliding_window(rows.clone(), match_pattern_3, &mut count);
    sliding_window(rows.clone(), match_pattern_4, &mut count);
    return count;
}

fn main() -> io::Result<()> {
    // Define the path to your input file
    let input_path = "input.txt";

    // Open the file
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    // Create a vector to hold rows of chars
    let mut rows: Vec<Vec<char>> = Vec::new();

    // Read lines from the file
    for line in reader.lines() {
        let row: Vec<char> = line?.chars().collect();
        rows.push(row);
    }

    // Prepare to search for the word "XMAS"
    let word: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let mut count = 0;

    // All possible directions
    let directions = [
        (1, 0),   // Down
        (0, 1),   // Right
        (1, 1),   // Down-Right
        (-1, 0),  // Up
        (0, -1),  // Left
        (-1, -1), // Up-Left
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
    ];

    // Iterate over each cell in the grid
    for i in 0..rows.len() {
        for j in 0..rows[0].len() {
            if rows[i][j] == 'X' {
                for &(dx, dy) in &directions {
                    if dfs(&rows, i as isize, j as isize, &word, 0, dx, dy) {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("{}", count);

    // Find the word "XMAS" in the shape of an X
    let x_mas_count = find_x_mas(rows);
    println!("{}", x_mas_count);

    Ok(())
}
