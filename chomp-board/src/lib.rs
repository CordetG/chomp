//! # Chomp Board
//!
//! THe chomp-board library crate provides implementation to handle the board for the game 'Chomp'.
//! This library manages a Board as a HashSet of positions. The primary functions are
//! for displaying, formatting, and adjusting the board based on a player's moves.

#![allow(unused_imports)]
#![allow(dead_code, unused_variables)]
use core::clone;
use std::collections::HashSet;

const COLMS: [&str; 11] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"];

/// Tuple type for the board position
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Position(String, u8);

/// Board Size type as m x n
#[derive(Clone)]
pub struct BoardSize(pub u8, pub u8);

#[derive(Clone)]
/// ## Board
///
/// The `Board` struct represents a game board with a specific size, state, and optional player move.
///
/// Properties:
///
/// * `size`: The `size` property in the `Board` struct represents the size of the board. It is a
/// struct that defines the dimensions of the board, such as the number of rows and columns.
/// * `state`: The `state` property in the `Board` struct represents the current positions occupied on
/// the board. It is a `HashSet` of `Position` elements, indicating the players' moves on the board.
/// * `player_move`: The `player_move` property in the `Board` struct represents the position where the
/// player intends to make a move on the board. It is an `Option` type, which means it can either
/// contain a `Position` value or be empty (`None`). This allows for flexibility in handling cases where
/// the player did not submit a move.
pub struct Board {
    size: BoardSize,
    pub state: HashSet<Position>,
    player_move: Option<Position>,
}

/// The `pub trait Game` defines a trait named `Game` that includes a method signature `fn new(def_size:
/// BoardSize) -> Self;`.
pub trait Game {
    /// The `fn new(def_size: BoardSize) -> Self;` is a method signature defined within the `Game` trait.
    /// This method signature specifies that any type implementing the `Game` trait must provide an
    /// implementation for a function named `new` that takes a `BoardSize` parameter and returns an instance
    /// of the implementing type (`Self`).
    fn new(def_size: BoardSize) -> Self;

    fn default_state(&mut self, size: BoardSize);
}

/// The `impl Board { ... }` block in the Rust code snippet is implementing additional methods for the
/// `Board` struct.
impl Board {
    pub fn chomped_board(current: &mut Board, chomped: Board) {
        for pos in current.state.difference(&chomped.state) {
            println!("Position: {:?}", pos);
        }
        todo!("create a new board state")
    }

    /// The function `format_board` takes a HashSet of Positions, formats them as strings, and joins them
    /// with commas.
    ///
    /// Arguments:
    ///
    /// * `to_display`: The `to_display` parameter is a reference to a `HashSet` containing elements of type
    /// `Position`.
    ///
    /// Returns:
    ///
    /// A string is being returned, which represents the formatted board with positions from the input
    /// `HashSet<Position>`.
    pub fn format_board(to_display: &HashSet<Position>) -> String {
        let display_board: Vec<String> = to_display
            .iter()
            .map(|p: &Position| (format!("{p}")))
            .collect();
        display_board.join(", ")
    }
}

/// The `impl std::fmt::Display for Position { ... }` block in Rust is implementing the `Display` trait
/// for the `Position` struct. By implementing the `Display` trait, you define how an instance of
/// `Position` should be formatted when converted to a string.
impl std::fmt::Display for Position {
    /// The function `fmt` is used to format a tuple with two elements into a string.
    ///
    /// Arguments:
    ///
    /// * `f`: The `f` parameter in the `fmt` function is a mutable reference to a `Formatter` instance from
    /// the `std::fmt` module. This `Formatter` is used to control the formatting of the output.
    ///
    /// Returns:
    ///
    /// The `fmt` method is returning a `Result<(), std::fmt::Error>`.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/// The `impl From<(String, String)> for BoardSize { ... }` block is implementing the `From`
/// trait for converting a tuple of two `String` values into a `BoardSize` struct.
impl From<(String, String)> for BoardSize {
    /// The function `from` takes a tuple of two strings and returns a `BoardSize` struct.
    ///
    /// Arguments:
    ///
    /// * `value`: (value: a tuple containing two strings)
    ///
    /// Returns:
    ///
    /// An instance of the `BoardSize` struct with the values from the tuple `value` passed as parameters.
    fn from(value: (String, String)) -> Self {
        BoardSize(
            value.0.parse::<u8>().unwrap(),
            value.1.parse::<u8>().unwrap(),
        )
    }
}

/// The `impl std::fmt::Display for BoardSize { ... }` block is implementing the `Display` trait
/// for the `BoardSize` struct. By implementing the `Display` trait, it defines how an instance of
/// `BoardSize` should be formatted when converted to a string.
impl std::fmt::Display for BoardSize {
    /// The function `fmt` is used to format and display the board size in a specific way.
    ///
    /// Arguments:
    ///
    /// * `f`: The `f` parameter in the `fmt` method is a mutable reference to a `Formatter` instance. This
    /// `Formatter` is used to control the formatting of the output. You can use methods provided by the
    /// `Formatter` to write formatted text to the output stream.
    ///
    /// Returns:
    ///
    /// The `fmt` method is returning a `Result<(), std::fmt::Error>`. This means that it is returning a
    /// `Result` enum where the success type is `()` (an empty tuple) and the error type is
    /// `std::fmt::Error`.
    ///
    /// # Example
    ///
    /// ```
    /// use chomp_board::*;
    ///
    /// let args: Vec<String> = vec!["--".to_string(),"4".to_string(),"5".to_string()];
    /// let arg_box: Box<Vec<String>> = Box::new(args);
    ///
    /// let game_box: Box<Vec<String>> = arg_box.clone();
    ///
    /// let BoardSize(m, n) = BoardSize::from((game_box[1].to_owned(), game_box[2].to_owned()));
    /// assert_eq!(format!("{}", BoardSize(m,n)), "Board Size: 4 x 5");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Board Size: {} x {}", self.0, self.1)
    }
}

/// This code snippet is implementing the `Game` trait for the `Board` struct..
impl Game for Board {
    /// The function `new` creates a new `Board` instance with a specified size and initializes the state
    /// and player move fields.
    ///
    /// Arguments:
    ///
    /// * `def_size`: The `def_size` parameter in the `new` function represents the size of the board. It is
    /// of type `BoardSize`, which specifies the dimensions or size of the game board, such as the
    /// number of rows and columns.
    ///
    /// Returns:
    ///
    /// A new `Board` instance is being returned with the specified `def_size`, an empty `HashSet` for the
    /// `state`, and `None` for the `player_move`.
    fn new(def_size: BoardSize) -> Board {
        Board {
            size: def_size,
            state: HashSet::new(),
            player_move: None,
        }
    }

    //#[allow(clippy::needless_range_loop)]
    fn default_state(&mut self, size: BoardSize) {
        let m = size.0 as usize;
        for (row, col) in COLMS.iter().enumerate().take(m) {
            self.state.insert(Position(col.to_string(), row as u8));
        }

        for pos in &self.state {
            println!("{} ", pos);
        }

        todo!("Add second dimension loop");
    }
}

// boilerplate template for generating tests ----------
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
