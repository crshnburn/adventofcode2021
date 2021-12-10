const INPUT: &str = include_str!("input");

fn main() {
    let scores = score_syntax_errors(INPUT);
    println!("Part 1: {}", scores.0);
    println!("Part 2: {}", scores.1);
}

fn score_syntax_errors(input: &str) -> (i32, i64) {
    let mut score = 0;
    let mut complete_scores: Vec<i64> = Vec::new();
    for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        let mut incomplete: bool = true;
        for c in line.trim().chars() {
            match c {
                '(' => {stack.push(c)}
                '[' => {stack.push(c)}
                '{' => {stack.push(c)}
                '<' => {stack.push(c)}
                ')' => {
                    let open = stack.pop().unwrap();
                    // println!("{}", open);
                    if open != '(' {
                        // println!("{}", line);
                        score += 3;
                        incomplete = false;
                        break;
                    }
                }
                ']' => {
                    let open = stack.pop().unwrap();
                    // println!("{}", open);
                    if open != '[' {
                        // println!("{}", line);
                        score += 57;
                        incomplete = false;
                        break;
                    }
                }
                '}' => {
                    let open = stack.pop().unwrap();
                    // println!("{}", open);
                    if open != '{' {
                        // println!("{}", line);
                        score += 1197;
                        incomplete = false;
                        break;
                    }
                }
                '>' => {
                    let open = stack.pop().unwrap();
                    // println!("{}", open);
                    if open != '<' {
                        // println!("{}", line);
                        score += 25137;
                        incomplete = false;
                        break;
                    }
                }
                _ => {}
            }
        }
        if incomplete {
            let mut complete_score = 0;
            while stack.len() > 0 {
                let c = stack.pop().unwrap();
                complete_score *= 5;
                match c {
                    '(' => {complete_score += 1}
                    '[' => {complete_score += 2}
                    '{' => {complete_score += 3}
                    _ => {complete_score += 4}
                }
            }
            complete_scores.push(complete_score);
        }
    }
    complete_scores.sort();
    // println!("{:?}", complete_scores);

    return (score, complete_scores[complete_scores.len()/2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_syntax_errors() {
        let input = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(score_syntax_errors(input), (26397, 288957))
    }
}