use std::cmp;

const INPUT: &str = include_str!("input");

fn main() {
    println!("Part 1: {}", calc_fuel(INPUT));
    println!("Part 2: {}", calc_fuel2(INPUT));
}

fn calc_fuel(input: &str) -> i32 {
    let mut pos: Vec<i32> = input.trim().split(",").map(|p| p.parse().unwrap()).collect();
    pos.sort();

    let median = statistical::median(&pos);
    return pos.iter().map(|p| (p - median).abs()).sum()
}

fn calc_fuel2(input: &str) -> i32 {
    let pos: Vec<f32> = input.trim().split(",").map(|p| p.parse().unwrap()).collect();

    let mean = statistical::mean(&pos).floor();
    let mean2 = statistical::mean(&pos).ceil();
    return cmp::min(pos.iter().map(|f| (f - mean).abs() as i32).map(|p| (p * (p+1))/2).sum(),
    pos.iter().map(|f| (f - mean2).abs() as i32).map(|p| (p * (p+1))/2).sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_fuel() {
        assert_eq!(calc_fuel("16,1,2,0,4,2,7,1,2,14"), 37)
    }

    #[test]
    fn test_calc_fuel2() {
        assert_eq!(calc_fuel2("16,1,2,0,4,2,7,1,2,14"), 168)
    }
}
