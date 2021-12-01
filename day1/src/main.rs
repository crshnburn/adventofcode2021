use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total_increase = 0;
    let mut total_increase_2 = 0;
    let mut prev = -1;
    let mut vec = Vec::new();
    let mut triple_sums = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(depth_str) = line {
                let depth = depth_str.parse().unwrap();
                vec.push(depth);
                // println!("Prev {}, Depth {}", prev, depth);
                if prev != -1 && depth > prev {
                    total_increase += 1;
                }
                prev = depth;
            }
        }
    }
    println!("Part 1: Total increases: {:?}", total_increase);
    for number in 0..vec.len()-2 {
        triple_sums.push(vec[number] + vec[number+1] + vec[number+2]);
    }
    for i in 1..triple_sums.len() {
        if triple_sums[i] > triple_sums[i-1] {
            total_increase_2 += 1;
        }
    }
    println!("Part 2: Total increases: {:?}", total_increase_2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
