use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut codes: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(code) = line {
                codes.push(code);
            }
        }
    }
    println!("Part 1: {}", calc(&codes));
    println!("Part 2: {}", calc2(&codes));
}

fn calc_totals(codes: &Vec<String>) -> Vec<(i32, i32)> {
    let mut totals: Vec<(i32, i32)> = Vec::new();
    for code in codes.iter() {
        let mut pos = 0;
        code.clone().drain(..).for_each(|c| {
            if let Some(res) = totals.get_mut(pos) {
                if c == '1' {
                    res.1 += 1;
                } else {
                    res.0 += 1;
                }
            } else {
                if c == '1' {
                    totals.push((0, 1));
                } else {
                    totals.push((1, 0));
                }
            }
            pos += 1;
        })
    }
    return totals;
}

fn calc(codes: &Vec<String>) -> i32 {
    let totals = calc_totals(codes);
    let mut epsilon: String = "".to_owned();
    let mut gamma = "".to_owned();
    for total in totals {
        if total.0 > total.1 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    // println!("Epsilon: {} Gamma: {}", epsilon, gamma);
    return i32::from_str_radix(&epsilon, 2).unwrap() * i32::from_str_radix(&gamma, 2).unwrap();
}

fn calc2(codes: &Vec<String>) -> i32 {
    let mut co2codes: Vec<String> = Vec::new();
    let mut lifecodes: Vec<String> = Vec::new();
    for code in codes.iter() {
        co2codes.push(code.to_string());
        lifecodes.push(code.clone());
    }
    
    let mut pos = 0;
    while lifecodes.len() > 1 {
        // println!("{:?}", lifecodes);
        let totals = calc_totals(&lifecodes);
        if totals[pos].0 > totals[pos].1 {
            lifecodes = lifecodes.into_iter().filter(|c| c.as_bytes()[pos] == 48).collect();
        } else {
            lifecodes = lifecodes.into_iter().filter(|c| c.as_bytes()[pos] == 49).collect();
        }
        pos += 1;
    }
    // println!("{:?}", lifecodes);
    pos = 0;
    while co2codes.len() > 1 {
        // println!("{:?}", co2codes);
        let totals = calc_totals(&co2codes);
        if totals[pos].0 > totals[pos].1 {
            co2codes = co2codes.into_iter().filter(|c| c.as_bytes()[pos] == 49).collect();
        } else {
            co2codes = co2codes.into_iter().filter(|c| c.as_bytes()[pos] == 48).collect();
        }
        pos += 1;
    }
    // println!("{:?}", co2codes);

    return i32::from_str_radix(&lifecodes[0], 2).unwrap() * i32::from_str_radix(&co2codes[0], 2).unwrap();
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
    fn test_calc() {
        let codes = vec![String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")];
        assert_eq!(calc(&codes), 198);
    }

    #[test]
    fn test_calc2() {
        let codes = vec![String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010")];
        assert_eq!(calc2(&codes), 230);
    }
}
