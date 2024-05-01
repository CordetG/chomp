#![allow(unused_imports)]
#![allow(dead_code, unused_variables)]
use std::collections::HashSet;

// Tuple type for the board position
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Position(String, u8);

// Board Size type as m x n
#[derive(Clone)]
pub struct BoardSize(pub String, pub String);

#[derive(Clone)]
pub struct Board {
    size: BoardSize,
    state: HashSet<Position>,
    player_move: Option<Position>,
}

pub trait Game {
    fn new(def_size: BoardSize) -> Self;
}

impl Board {
    pub fn chomped_board(current: &mut Board, chomped: Board) {
        for pos in current.state.difference(&chomped.state) {
            println!("Position: {:?}", pos);
        }
        todo!("create a new board state")
    }

    pub fn format_board(to_display: &HashSet<String>) -> String {
        let display_board: Vec<&str> = to_display.iter().map(String::as_ref).collect();
        display_board.join(", ")
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl From<(String, String)> for BoardSize {
    fn from(value: (String, String)) -> Self {
        BoardSize(value.0, value.1)
    }
}

impl std::fmt::Display for BoardSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Board Size: {} x {}", self.0, self.1)
    }
}

impl Game for Board {
    fn new(def_size: BoardSize) -> Board {
        Board {
            size: def_size,
            state: HashSet::new(),
            player_move: None,
        }
    }
}

// boilerplate
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
