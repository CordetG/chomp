#![allow(unused_imports)]
#![allow(dead_code, unused_variables)]
use std::collections::HashSet;

// Tuple type for the board position
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Position(String, u8);

// Board Size type
#[derive(Clone)]
pub struct BoardSize(u64);

#[derive(Clone)]
pub struct Board {
    size: BoardSize,
    state: HashSet<Position>,
    player_move: Position,
}

impl Board {
    pub fn new(size: BoardSize, player_move: Position) -> Self {
        Board {
            size,
            state: HashSet::new(),
            player_move,
        }
    }

    pub fn chomped_board(current: &mut Board, chomped: Board) {
        for pos in current.state.difference(&chomped.state) {
            println!("Position: {:?}", pos);
        }
        todo!("create a new board state")
    }

    pub fn display_board() {}
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl std::fmt::Display for BoardSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Board Size: {}", self.0)
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
