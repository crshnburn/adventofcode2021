use std::collections::HashSet;

const INPUT: &str = include_str!("input");

fn main() {
    println!("Part 1: {}", count_easy(INPUT.lines().collect()));
    println!("Part 2: {}", calc_total(INPUT.lines().collect()));
}

fn count_easy(lines: Vec<&str>) -> usize {
    // println!("{:?}", lines);
    let mut outputs: Vec<&str> = Vec::new();
    for line in lines {
        outputs.push(line.split("|").collect::<Vec<&str>>()[1]);
    }
    let mut numbers: Vec<&str> = Vec::new();
    for output in outputs {
        let number: Vec<&str> = output.trim().split_whitespace().collect();
        for num in number {
            numbers.push(num);
        }
    }
    return numbers.iter().filter(|n| n.len()==2 || n.len()==3 || n.len()==4 || n.len()==7).collect::<Vec<&&str>>().len();
}

fn calc_total(lines: Vec<&str>) -> i32 {
    return lines.iter().map(|line| calc_line(line)).sum();
}

fn calc_line(line: &str) -> i32 {
    let (input, values) = line.split_once(" | ").unwrap();
    let mut patterns: Vec<_> = input.trim().split_whitespace().map(|d| d.chars().collect::<HashSet<_>>()).collect();
    patterns.sort_unstable_by_key(|p| p.len());
    let digits: Vec<_> = values.trim().split_whitespace().map(|d| d.chars().collect::<HashSet<_>>()).collect();
    
    return decode(&patterns, &digits[0]) * 1000 + 
            decode(&patterns, &digits[1]) * 100 + 
            decode(&patterns, &digits[2]) * 10 +
            decode(&patterns, &digits[3]);
}

fn decode(patterns: &[HashSet<char>], digit: &HashSet<char>) -> i32 {
    if digit.len() == patterns[0].len() {
        return 1;
    }
    if digit.len() == patterns[1].len() {
        return 7;
    }
    if digit.len() == patterns[2].len() {
        return 4;
    }
    if digit.len() == patterns[9].len() {
        return 8;
    }
    if digit.len() == 5 {
        if patterns[0].difference(digit).count() == 0 {
            return 3;
        } else if patterns[2].difference(digit).count() == 1 {
            return 5;
        } else {
            return 2;
        }
    } else {
        if patterns[0].difference(digit).count() > 0 {
            return 6;
        } else if patterns[2].difference(digit).count() > 0{
            return 0;
        } else {
            return 9;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_easy() {
        let test_input = vec!("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        assert_eq!(count_easy(test_input), 26);
    }

    #[test]
    fn test_calc_line() {
        assert_eq!(calc_line("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
    }

    #[test]
    fn test_count_total() {
        let test_input = vec!("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce");
        assert_eq!(calc_total(test_input), 61229);
    }
}
