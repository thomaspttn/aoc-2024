use std::fs::File;
use std::io::{self, BufRead};

pub fn run_path_algo(
    matrix: Vec<Vec<char>>,
    start_loc: (isize, isize),
    start_direction: char,
    max_steps: Option<usize>,
) -> (Vec<Vec<bool>>, bool) {
    let mut loc = start_loc;
    let mut direction = start_direction;
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    visited[loc.0 as usize][loc.1 as usize] = true;
    let mut out_of_bounds = false;

    let mut steps = 0;
    while !out_of_bounds {
        if let Some(max_steps) = max_steps {
            if steps >= max_steps {
                return (visited, false);
            }
            steps += 1;
        }
        if direction == '^' {
            if loc.0 == 0 {
                out_of_bounds = true;
            } else if matrix[loc.0 as usize - 1][loc.1 as usize] == '#' {
                direction = '>';
            } else {
                loc = (loc.0 - 1, loc.1);
                visited[loc.0 as usize][loc.1 as usize] = true;
            }
        } else if direction == 'v' {
            if loc.0 == matrix.len() as isize - 1 {
                out_of_bounds = true;
            } else if matrix[loc.0 as usize + 1][loc.1 as usize] == '#' {
                direction = '<';
            } else {
                loc = (loc.0 + 1, loc.1);
                visited[loc.0 as usize][loc.1 as usize] = true;
            }
        } else if direction == '<' {
            if loc.1 == 0 {
                out_of_bounds = true;
            } else if matrix[loc.0 as usize][loc.1 as usize - 1] == '#' {
                direction = '^';
            } else {
                loc = (loc.0, loc.1 - 1);
                visited[loc.0 as usize][loc.1 as usize] = true;
            }
        } else if direction == '>' {
            if loc.1 == matrix[0].len() as isize - 1 {
                out_of_bounds = true;
            } else if matrix[loc.0 as usize][loc.1 as usize + 1] == '#' {
                direction = 'v';
            } else {
                loc = (loc.0, loc.1 + 1);
                visited[loc.0 as usize][loc.1 as usize] = true;
            }
        }
    }
    return (visited, true);
}

pub fn check_if_obstruction_causes_infinite_loop(
    obs_x: isize,
    obs_y: isize,
    mut matrix: Vec<Vec<char>>,
    loc: (isize, isize),
    direction: char,
) -> bool {
    // first, check if obsruction is on a # or on the starting position: that's a no!
    if matrix[obs_x as usize][obs_y as usize] == '#' || (obs_x, obs_y) == loc {
        return false;
    }

    // place the obstruction
    matrix[obs_x as usize][obs_y as usize] = '#';

    // run the path algo with max steps
    let (_, terminated) = run_path_algo(matrix, loc, direction, Some(17000));

    return !terminated;
}

fn main() -> io::Result<()> {
    // Define the path to your input file
    let input_path = "input.txt";

    // Open the file
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    // Vec to store the lines
    let mut matrix = Vec::new();

    let mut loc: (isize, isize) = (0, 0);
    let mut direction = '^';

    // index based loop over the lines
    for (i, line) in reader.lines().enumerate() {
        // for each char in line
        let mut line_vec = Vec::new();
        for (j, c) in line.unwrap().chars().enumerate() {
            // if char is one of ^ v < > update the starting point
            if c == '^' || c == 'v' || c == '<' || c == '>' {
                loc = (i as isize, j as isize);
                direction = c;
            }
            line_vec.push(c);
        }
        matrix.push(line_vec);
    }

    println!("Starting Point: {:?}", loc);
    println!("Starting Direction: {:?}", direction);

    // run the path algo
    let (visited, _) = run_path_algo(matrix.clone(), loc, direction, None);

    // turn visited into .'s and X's
    // store all X's in a set

    let mut visited_coords = std::collections::HashSet::new();

    let mut true_visited_count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if visited[i][j] {
                matrix[i][j] = 'X';
                true_visited_count += 1;
                visited_coords.insert((i as isize, j as isize));
            } else if matrix[i][j] == '#' {
                matrix[i][j] = '#';
            } else {
                matrix[i][j] = '.';
            }
        }
    }

    // print the matrix cleanly
    //for i in 0..matrix.len() {
    //    for j in 0..matrix[0].len() {
    //        print!("{}", matrix[i][j]);
    //    }
    //    println!();
    //}

    println!("True Visited Count: {}", true_visited_count);

    // Part 2
    // for each x,y in the matrix, check if obstruction causes infinite loop
    let mut infinite_loop_count = 0;
    //for i in 0..matrix.len() {
    //    println!("Checking row: {}", i);
    //    for j in 0..matrix[0].len() {
    //        if check_if_obstruction_causes_infinite_loop(
    //            i as isize,
    //            j as isize,
    //            matrix.clone(),
    //            loc,
    //            direction,
    //        ) {
    //            infinite_loop_count += 1;
    //        }
    //    }
    //}
    for (i, j) in visited_coords.iter() {
        if check_if_obstruction_causes_infinite_loop(*i, *j, matrix.clone(), loc, direction) {
            infinite_loop_count += 1;
        }
    }
    println!("Infinite Loop Count: {}", infinite_loop_count);

    Ok(())
}
