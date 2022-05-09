use std::cmp::Ordering;
use std::str::FromStr;
use std::{fs, io};

use board::{Board, RawBoard, Status, DIMENSION};
use min_priority_queue::MinPriorityQueue;

mod board;
mod min_priority_queue;

#[derive(Clone)]
struct Step {
    level: usize,
    cost: usize,
    board: Board,
}

impl Step {
    pub fn new(board: &Board, level: usize, heuristic: fn(&Board, usize) -> usize) -> Step {
        Step {
            level,
            cost: heuristic(board, level),
            board: board.clone(),
        }
    }
}

impl PartialEq<Self> for Step {
    fn eq(&self, other: &Self) -> bool {
        self.board.eq(&other.board)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.board.partial_cmp(&other.board)
    }
}

fn heuristic(board: &Board, level: usize) -> usize {
    let row_dis = DIMENSION - board.player_point.row - 1;
    let col_dis = DIMENSION - board.player_point.col - 1;
    level + row_dis + col_dis
}

fn solver(raw_board: &RawBoard) {
    let mut queue = MinPriorityQueue::<Step>::new();
    let mut visited_boards = Vec::<Board>::new();
    let mut steps = Vec::<Step>::new();

    queue.push(Step::new(&Board::new(raw_board), 0, heuristic));

    while !queue.is_empty() {
        match queue.pop() {
            None => {
                panic!("Invalid operation exceeded.");
            }
            Some(step) => {
                let level = step.level;

                if level >= steps.len() {
                    steps.push(step.clone());
                } else {
                    steps[level] = step.clone();
                }

                if step.board.is_goal() {
                    break;
                }

                visited_boards.push((&step.board).clone());

                match queue.peek() {
                    None => {
                        for next_board in &step.board.next_boards() {
                            if !visited_boards.contains(next_board) {
                                queue.push(Step::new(&next_board, level + 1, heuristic));
                            }
                        }
                    }
                    Some(min_step) => {
                        let min_cost = heuristic(&min_step.board, min_step.level);

                        for next_board in step.board.next_boards() {
                            let next_cost = heuristic(&next_board, level + 1);

                            if !visited_boards.contains(&next_board) && next_cost <= min_cost {
                                queue.push(Step::new(&next_board, level + 1, heuristic));
                            }
                        }
                    }
                }
            }
        }
    }

    if !steps.last().unwrap().board.is_goal() {
        println!("Cannot find any paths.")
    } else {
        println!("Move count: {}", steps.len() - 1);

        for (i, step) in steps.iter().enumerate() {
            println!("Step {}: ", i);
            println!("{}", step.board.to_str());
        }
    }
}

fn main() {
    let mut input_buffer = String::new();

    io::stdin()
        .read_line(&mut input_buffer)
        .expect("Failed to read lines from console");

    let content = fs::read_to_string(input_buffer.trim_end()).expect("Failed to read the file");
    let mut data = [[Status::None; DIMENSION]; DIMENSION] as RawBoard;

    for (i, buf) in content.split_whitespace().enumerate() {
        let row = i / DIMENSION;
        let col = i % DIMENSION;

        data[row][col] = Status::from_str(buf).expect("Invalid board file");
    }

    solver(&data);
}
