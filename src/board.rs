use std::cmp::Ordering;
use std::str::FromStr;

pub const DIMENSION: usize = 5;

#[derive(PartialEq, Copy, Clone)]
pub enum Status {
    None,
    Wall,
    Player,
}

pub type RawBoard = [[Status; DIMENSION]; DIMENSION];

#[derive(Copy, Clone)]
pub struct PlayerPoint {
    pub row: usize,
    pub col: usize,
}

#[derive(Clone)]
pub struct Board {
    data: Vec<Status>,
    pub player_point: PlayerPoint,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Status, Self::Err> {
        match s {
            "2" => Ok(Status::Player),
            "1" => Ok(Status::Wall),
            "0" => Ok(Status::None),
            _ => Err(()),
        }
    }
}

impl Board {
    pub fn new(data: &RawBoard) -> Board {
        let mut board = Board {
            data: data
                .to_vec()
                .iter()
                .map(|v| Vec::<Status>::from(v as &[Status]))
                .flatten()
                .collect(),
            player_point: PlayerPoint {
                row: usize::MAX,
                col: usize::MAX,
            },
        };

        for (i, _) in board.data.iter().enumerate() {
            if board.data[i] == Status::Player {
                board.player_point.row = i / DIMENSION;
                board.player_point.col = i % DIMENSION;
                return board;
            }
        }

        return board;
    }

    pub fn is_goal(&self) -> bool {
        self.player_point.row == DIMENSION - 1 && self.player_point.col == DIMENSION - 1
    }

    pub fn is_valid(&self) -> bool {
        self.player_point.row != usize::MAX && self.player_point.col != usize::MAX
    }

    pub fn next_boards(&self) -> Vec<Board> {
        let mut next_board_vec = Vec::<Board>::with_capacity(4);

        if !self.is_valid() {
            next_board_vec
        } else {
            let player_point = &self.player_point;
            let index = player_point.row * DIMENSION + player_point.col;

            if player_point.row > 0 && self.data[index - DIMENSION] != Status::Wall {
                let mut new_board = self.clone();
                new_board.data.swap(index - DIMENSION, index);
                new_board.player_point.row = player_point.row - 1;
                next_board_vec.push(new_board);
            }

            if player_point.col > 0 && self.data[index - 1] != Status::Wall {
                let mut new_board = self.clone();
                new_board.data.swap(index - 1, index);
                new_board.player_point.col = player_point.col - 1;
                next_board_vec.push(new_board);
            }

            if player_point.row + 1 < DIMENSION && self.data[index + DIMENSION] != Status::Wall {
                let mut new_board = self.clone();
                new_board.data.swap(index + DIMENSION, index);
                new_board.player_point.row = player_point.row + 1;
                next_board_vec.push(new_board);
            }

            if player_point.col + 1 < DIMENSION && self.data[index + 1] != Status::Wall {
                let mut new_board = self.clone();
                new_board.data.swap(index + 1, index);
                new_board.player_point.col = player_point.col + 1;
                next_board_vec.push(new_board);
            }

            next_board_vec.shrink_to_fit();
            next_board_vec
        }
    }

    pub fn to_str(&self) -> String {
        let mut builder = String::new();

        self.data.iter().enumerate().for_each(|(i, &status)| {
            builder.push_str(match status {
                Status::None => " _ ",
                Status::Wall => " X ",
                Status::Player => " O ",
            });

            if (i + 1) % DIMENSION == 0 {
                builder.push('\n');
            }
        });

        builder.shrink_to_fit();

        return builder;
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl PartialOrd for Board {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_cost = self.player_point.row + self.player_point.col;
        let other_cost = other.player_point.row + other.player_point.col;
        other_cost.partial_cmp(&self_cost)
    }
}
