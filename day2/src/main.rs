use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut moves: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(mov) = line {
                moves.push(mov);
            }
        }
    }
    println!("Part 1: {}", move_sub(&moves));
    println!("Part 2: {}", move_with_aim(&moves));
}

fn move_sub(moves: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;
    for line in moves {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "forward" => {horizontal += parts[1].parse::<i32>().unwrap()}
            "up" => {vertical -= parts[1].parse::<i32>().unwrap()}
            "down" => {vertical += parts[1].parse::<i32>().unwrap()}
            _ => {}
        } 
    }
    return horizontal * vertical;
}

fn move_with_aim(moves: &Vec<String>) -> i32 {
    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;
    for line in moves {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "forward" => {
                let val = parts[1].parse::<i32>().unwrap();
                horizontal += val;
                vertical += val * aim;
            }
            "up" => {aim -= parts[1].parse::<i32>().unwrap()}
            "down" => {aim += parts[1].parse::<i32>().unwrap()}
            _ => {}
        } 
    }
    return horizontal * vertical;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move() {
        let moves = vec![String::from("forward 5"),
                        String::from("down 5"),
                        String::from("forward 8"),
                        String::from("up 3"),
                        String::from("down 8"),
                        String::from("forward 2")];
        assert_eq!(move_sub(&moves), 150)
    }

    #[test]
    fn test_move_with_aim() {
        let moves = vec![String::from("forward 5"),
                        String::from("down 5"),
                        String::from("forward 8"),
                        String::from("up 3"),
                        String::from("down 8"),
                        String::from("forward 2")];
        assert_eq!(move_with_aim(&moves), 900)
    }
}

