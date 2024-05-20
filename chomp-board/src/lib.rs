//! # Chomp Board
//!
//! THe chomp-board library crate provides implementation to handle the board for the game 'Chomp'.
//! This library manages a Board as a HashSet of positions. The primary functions are
//! for displaying, formatting, and adjusting the board based on a player's moves.

#![allow(unused_imports)]
#![allow(dead_code, unused_variables)]
use core::clone;
use itertools::Itertools;
use std::collections::HashSet;

/// The line `const COLMS: [&str; 10] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];` is
/// declaring a constant array named `COLMS` that contains string slices (`&str`) with a length of 10.
/// Each element of the array represents a column identifier in the context of the Chomp game board. The
/// identifiers are alphabetical letters from "a" to "j", which are commonly used to label columns on a
/// game board or grid.
const COLMS: [&str; 10] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

/// Tuple type for the board position
#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct Position(pub char, pub u8);

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
    pub player_move: Option<Position>,
}

/// The `pub trait Game` defines a trait named `Game` that includes a method signature `fn new(def_size:
/// BoardSize) -> Self;`.
pub trait Game {
    /// The `fn new(def_size: BoardSize) -> Self;` is a method signature defined within the `Game` trait.
    /// This method signature specifies that any type implementing the `Game` trait must provide an
    /// implementation for a function named `new` that takes a `BoardSize` parameter and returns an instance
    /// of the implementing type (`Self`).
    fn new(def_size: BoardSize) -> Self;

    /// The `fn default_state(&mut self, size: BoardSize);` function signature within the `Game` trait is
    /// defining a method named `default_state` that takes a mutable reference to `self` (the `Board`
    /// instance) and a `BoardSize` parameter named `size`.
    fn default_state(&mut self, size: BoardSize);
}

/// The `impl Board { ... }` block is implementing additional methods for the `Board` struct.
impl Board {
    /// The function `chomped_board` takes two Board structs, compares their states, and returns a HashSet
    /// of Positions that are in the first Board but not in the second.
    ///
    /// Arguments:
    ///
    /// * `current`: The `current` parameter is a mutable reference to a `Board` struct.
    /// * `chomped`: The `chomped` parameter in the `chomped_board` function is of type `HashSet<Position>`. It
    /// represents the positions to be removed that will be used to update the `current` board state.
    ///
    /// Returns:
    ///
    /// The function `chomped_board` returns a `HashSet<Position>` containing the positions that are in the
    /// `current` board state but not in the `chomped` board state.
    ///
    /// # Example
    ///
    /// ```
    /// use chomp_board::*;
    /// use std::collections::HashSet;
    ///
    /// let args: Vec<String> = vec!["--".to_string(),"3".to_string(),"4".to_string()];
    /// let arg_box: Box<Vec<String>> = Box::new(args);
    ///
    /// let game_box: Box<Vec<String>> = arg_box.clone();
    ///
    /// let BoardSize(m, n) = BoardSize::from((game_box[1].to_owned(), game_box[2].to_owned()));
    ///
    /// let mut chomp_bar: Board = chomp_board::Game::new(BoardSize(m, n));
    /// <chomp_board::Board as chomp_board::Game>::default_state(&mut chomp_bar, BoardSize(m, n));
    ///
    /// let chomped_pieces: HashSet<Position> = HashSet::from([Position('c', 2), Position('d', 2), Position('c',3), Position('d', 3)]);
    ///
    /// let new_board: HashSet<Position> = Board::chomped_board(&mut chomp_bar, chomped_pieces);
    ///
    /// assert_eq!(new_board.contains(&Position('a', 1)), true);
    /// assert_eq!(new_board.contains(&Position('c', 2)), false);
    /// ```
    pub fn chomped_board(current: &mut Board, chomped: HashSet<Position>) {
        let new_state: HashSet<Position> = current.state.difference(&chomped).cloned().collect();
        current.state = new_state;
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
    pub fn format_board(to_display: &Board) {
        let mut board_vec: Vec<_> = to_display.state.iter().collect();
        board_vec.sort();
        let mut col: Vec<_> = board_vec.iter().map(|pos| pos.0).collect();
        col = col.into_iter().unique().collect();
        println!("Columns: {:?}", col);
        let mut row: Vec<_> = board_vec.iter().map(|pos| pos.1).collect();
        row = row.into_iter().unique().collect();
        println!("Rows: {:?}", row);
        // Find the dimensions m x n of the matrix
        let msize: usize = row.len();
        let nsize: usize = col.len();
        let mut s = 0;
        let mut t = 0;

        let mut matrix: Vec<Vec<(char, u8)>> = vec![vec![(' ', 0); nsize]; msize];

        #[allow(clippy::needless_range_loop, clippy::explicit_counter_loop)]
        for r in &row {
            for c in &col {
                matrix[s][t] = (*c, *r);
                t += 1;
            }
            s += 1;
            t = 0;
        }

        println!("\n (a, 1) is poisoned! \n");
        for i in &matrix {
            println!("{:?}", i);
        }
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

impl From<(String, String)> for Position {
    fn from(pos: (String, String)) -> Self {
        Position(
            pos.0.to_lowercase().chars().next().unwrap(),
            pos.1.parse::<u8>().expect("u8 not returned\n"),
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

    //m = row, n = col
    fn default_state(&mut self, size: BoardSize) {
        let m: usize = size.0 as usize;
        let n: usize = size.1 as usize;

        for row in 1..=m {
            for (col, alpha) in COLMS.iter().take(n).enumerate() {
                self.state.insert(Position(
                    alpha.to_string().chars().next().unwrap(),
                    row as u8,
                ));
            }
        }

        Board::format_board(&self.clone());
    }
}
