use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(input) = line {
                inputs.push(input);
            }
        }
    }
    // println!("Part 1: {}", find_paths(inputs));
    println!("Part 2: {}", find_all_paths(inputs));
}

#[derive(Debug)]
struct CoOrdinate {
    x: i32,
    y: i32
}

fn parse_line(line: String) -> (CoOrdinate, CoOrdinate) {
    let parts: Vec<&str> = line.split(" -> ").collect();
    let start: Vec<&str> = parts[0].split(",").collect();
    let end: Vec<&str> = parts[1].split(",").collect();
    return (CoOrdinate{x: start[0].parse().unwrap(), y: start[1].parse().unwrap()},
            CoOrdinate{x: end[0].parse().unwrap(), y: end[1].parse().unwrap()})
}

fn find_paths(lines: Vec<String>) -> i32 {
    let mut grid = Box::new([[0; 1000]; 1000]);
    for line in lines {
        let path = parse_line(line);
        if path.0.x == path.1.x || path.0.y == path.1.y {
            // println!("{:?}", path);
            for y in cmp::min(path.0.y,path.1.y)..cmp::max(path.0.y,path.1.y)+1 {
                for x in cmp::min(path.0.x,path.1.x)..cmp::max(path.0.x,path.1.x)+1 {
                    grid[y as usize][x as usize] += 1
                }
            }
        } 
    }
    // println!("{:?}",grid);
    let mut totals = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if grid[y][x] > 1 {
                totals += 1;
            }
        }
    }
    return totals;
}

fn find_all_paths(lines: Vec<String>) -> i32 {
    let mut grid = Box::new([[0; 1000]; 1000]);
    for line in lines {
        let path = parse_line(line);
        if path.0.x == path.1.x || path.0.y == path.1.y {
            for y in cmp::min(path.0.y,path.1.y)..cmp::max(path.0.y,path.1.y)+1 {
                for x in cmp::min(path.0.x,path.1.x)..cmp::max(path.0.x,path.1.x)+1 {
                    grid[y as usize][x as usize] += 1
                }
            }
        } else {
            if path.0.y < path.1.y {
                if path.0.x < path.1.x {
                    for i in 0..path.1.x-path.0.x+1 {
                        grid[(path.0.y+i) as usize][(path.0.x+i) as usize] += 1
                    }
                } else {
                    for i in 0..path.0.x-path.1.x+1 {
                        grid[(path.0.y+i) as usize][(path.0.x-i) as usize] += 1
                    }
                }
            } else {
                if path.0.x < path.1.x {
                    for i in 0..path.1.x-path.0.x+1 {
                        grid[(path.0.y-i) as usize][(path.0.x+i) as usize] += 1
                    }
                } else {
                    for i in 0..path.0.x-path.1.x+1 {
                        grid[(path.0.y-i) as usize][(path.0.x-i) as usize] += 1
                    }
                }
            }
        }
    }
    // println!("{:?}",grid);
    let mut totals = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            if grid[y][x] > 1 {
                totals += 1;
            }
        }
    }
    return totals;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let result = parse_line(String::from("0,9 -> 5,9"));
        assert_eq!(result.0.x, 0);
        assert_eq!(result.0.y, 9);
        assert_eq!(result.1.x, 5);
        assert_eq!(result.1.y, 9);
    }

    #[test]
    fn test_find_paths() {
        let lines = vec!{String::from("0,9 -> 5,9"),
                            String::from("8,0 -> 0,8"),
                            String::from("9,4 -> 3,4"),
                            String::from("2,2 -> 2,1"),
                            String::from("7,0 -> 7,4"),
                            String::from("6,4 -> 2,0"),
                            String::from("0,9 -> 2,9"),
                            String::from("3,4 -> 1,4"),
                            String::from("0,0 -> 8,8"),
                            String::from("5,5 -> 8,2")};
        assert_eq!(find_paths(lines), 5);
    }

    #[test]
    fn test_find_all_paths() {
        let lines = vec!{String::from("0,9 -> 5,9"),
                            String::from("8,0 -> 0,8"),
                            String::from("9,4 -> 3,4"),
                            String::from("2,2 -> 2,1"),
                            String::from("7,0 -> 7,4"),
                            String::from("6,4 -> 2,0"),
                            String::from("0,9 -> 2,9"),
                            String::from("3,4 -> 1,4"),
                            String::from("0,0 -> 8,8"),
                            String::from("5,5 -> 8,2")};
        assert_eq!(find_all_paths(lines), 12);
    }
}
