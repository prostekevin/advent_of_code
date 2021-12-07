use std::{str::Lines };

#[derive(Debug)]
struct Board {
    mat: [[(bool , i32);5];5],
}

impl Board {
    fn new() -> Self {
        Board {
            mat : [[(false, 0);5];5],
        }
    }

    fn print(&self) {
        for row in self.mat.iter() {
            for col in row.iter() {
                print!("{:?} ", col)
            }
            println!{""}
        }
        println!("")
    }
}

fn parse_input(data: &str) -> (Vec<Board>, Vec<i32>) {

    let mut lines = data.lines();

    let numbers: Vec<_> = lines.next().unwrap().split(',').map(|n| n.parse::<i32>().unwrap()).collect();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        boards.push(parse_mat(&mut lines))
    }
    (boards, numbers)
}

fn parse_mat(lines: &mut Lines) -> Board {
    let mut board = Board::new();
    for row in 0..5 {
        lines.next().unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .enumerate()
                    .for_each(|(col, num)| board.mat[row][col] = (false ,num))
    }
    board
}
fn mark_and_check(board: &mut Board, num: i32) -> bool {
    for row in 0..board.mat.len() {
        for col in 0..board.mat[row].len(){
            if board.mat[row][col].1 == num {
                board.mat[row][col].0 = true;
                return check_win(&board, row, col);
            }
        }
    }
    false
}
fn check_win(board: &Board, row: usize, col: usize) -> bool {
    (0..board.mat.len()).all(|ac_row| board.mat[ac_row][col].0) || 
        (0..board.mat[row].len()).all(|ac_col| board.mat[row][ac_col].0)
}

fn sum_of_unmarked(board: &Board) -> i32 {
    board.mat.iter().flatten().fold(0, |acc, &(mark, cur_n)| match mark {
        true => acc,
        _ => acc + cur_n,
    })
}

fn first_star(input: &str) {
    let (mut boards, numbers) = parse_input(input);
    for num in numbers{
        for board in &mut boards {
            if mark_and_check(board, num) {
                let sum = sum_of_unmarked(board);
                println!("answer is {}", sum * num);
                return;
            }
        }
    }
}

fn second_star(input: &str) {
    let (mut boards, numbers) = parse_input(input);
    let mut board_refs: Vec<&mut Board> = boards.iter_mut().collect();
    for num in numbers {
        let remaining_boards = board_refs.len();
        let mut filtered_boards: Vec<&mut Board> = Vec::with_capacity(remaining_boards);
        for board in board_refs {
            if mark_and_check(board, num){
                if remaining_boards == 1 {
                    let sum = sum_of_unmarked(board);
                    println!("answer is {}", sum * num);
                    return;
                    }
                } else {
                    filtered_boards.push(board);
                }
            
        }
        board_refs = filtered_boards;
    }
}

fn main() {
    let data = include_str!("test.in");
    first_star(data);
    second_star(data);  
}
