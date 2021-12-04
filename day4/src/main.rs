use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct BingoSquare(String, bool);

#[derive(Default)]
#[derive(Debug)]
struct BingoBoard {
    board: Vec<BingoSquare>
}

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(input) = line {
                inputs.push(input);
            }
        }
    }
    println!("Result: {}", bingo(inputs));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn bingo(input: Vec<String>) -> i32 {
    let called_numbers = input[0].split(',');

    let mut boards: Vec<BingoBoard> = Vec::new();
    let boards_input = &input[2..];
    let mut line_number = 0;
    let mut board: BingoBoard = Default::default();
    for line in boards_input {
        if line_number < 5 {
            let numbers = line.split_whitespace();
            for number in numbers {
                board.board.push(BingoSquare(String::from(number), false));
            }
            line_number += 1
        } else {
            line_number = 0;
            boards.push(board);
            board = Default::default();
        }
    }
    boards.push(board);
    // println!("{:?}", boards);
    for number in called_numbers {
        for board in boards.iter_mut() {
            for square in (board.board).iter_mut() {
                if square.0 == number {
                    *square = BingoSquare(String::from(number), true);
                }
            }
        }
        //Part1
        // for board in boards.iter_mut() {
        //     if is_winner(board) {
        //         return calc_board(board) * number.parse::<i32>().unwrap();
        //     }
        // }
        if boards.len() > 1 {
            boards = boards.into_iter().filter(|b| !is_winner(b)).collect();
        } else {
            if is_winner(&boards[0]) {
                return calc_board(&boards[0]) * number.parse::<i32>().unwrap();
            }
        }
    }
    return 0;
}

fn is_winner(board: &BingoBoard) -> bool {
    return board.board[0].1 && board.board[1].1 && board.board[2].1 && board.board[3].1 && board.board[4].1 ||
           board.board[5].1 && board.board[6].1 && board.board[7].1 && board.board[8].1 && board.board[9].1 ||
           board.board[10].1 && board.board[11].1 && board.board[12].1 && board.board[13].1 && board.board[14].1 ||
           board.board[15].1 && board.board[16].1 && board.board[17].1 && board.board[18].1 && board.board[19].1 ||
           board.board[20].1 && board.board[21].1 && board.board[22].1 && board.board[23].1 && board.board[24].1 ||
           board.board[0].1 && board.board[5].1 && board.board[10].1 && board.board[15].1 && board.board[20].1 ||
           board.board[1].1 && board.board[6].1 && board.board[11].1 && board.board[16].1 && board.board[21].1 ||
           board.board[2].1 && board.board[7].1 && board.board[12].1 && board.board[17].1 && board.board[22].1 ||
           board.board[3].1 && board.board[8].1 && board.board[13].1 && board.board[18].1 && board.board[23].1 ||
           board.board[4].1 && board.board[9].1 && board.board[14].1 && board.board[19].1 && board.board[24].1;
}

fn calc_board(board: &BingoBoard) -> i32 {
    let mut total: i32 = 0;
    for square in board.board.iter() {
        if !square.1 {
            total += square.0.parse::<i32>().unwrap();
        }
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo() {
        let game = vec!{String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"),
            String::from(""),
            String::from("22 13 17 11  0"),
            String::from(" 8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from(" 6 10  3 18  5"),
            String::from(" 1 12 20 15 19"),
            String::from(""),
            String::from(" 3 15  0  2 22"),
            String::from(" 9 18 13 17  5"),
            String::from("19  8  7 25 23"),
            String::from("20 11 10 24  4"),
            String::from("14 21 16 12  6"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from(" 2  0 12  3  7")};
        // assert_eq!(bingo(game), 4512); //Part1
        assert_eq!(bingo(game), 1924); //Part2
    }
}