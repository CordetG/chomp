//! # Chomp Board
//!
//! THe chomp-board library crate provides implementation to handle the board for the game 'Chomp'.
//! This library manages a Board as a HashSet of positions. The primary functions are
//! for displaying, formatting, and adjusting the board based on a player's moves.

use itertools::Itertools;
use std::collections::HashSet;

/// The line `const COLMS: [&str; 10] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];` is
/// declaring a constant array named `COLMS` that contains string slices (`&str`) with a length of 10.
/// Each element of the array represents a column identifier in the context of the Chomp game board. The
/// identifiers are alphabetical letters from "a" to "j", which are commonly used to label columns on a
/// game board.
const COLMS: [&str; 10] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

/// Tuple type for the board position
#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
pub struct Position(pub char, pub u8);

/// Board Size type as m x n
#[derive(Clone)]
pub struct BoardSize(pub u8, pub u8);

#[derive(Clone)]
/// The `Board` struct represents a game board with a state containing positions and an optional player
/// move.
///
/// Properties:
///
/// * `state`: The `state` property in the `Board` struct represents the current positions occupied on
/// the board. It is a `HashSet` of `Position` objects, indicating where the players have placed their
/// pieces on the board.
/// * `player_move`: The `player_move` property in the `Board` struct represents the position where the
/// player intends to make a move on the board. It is an `Option<Position>`, which means it can either
/// be `Some(Position)` indicating a specific position chosen by the player, or `None` if
pub struct Board {
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
    fn new() -> Self;

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
    pub fn chomped_board(current: &mut Board, chomped: HashSet<Position>) -> HashSet<Position> {
        let new_state: HashSet<Position> = current.state.difference(&chomped).cloned().collect();

        new_state
    }

    /// The `format_board` function sorts and formats a board represented as a matrix of
    /// characters and integers.
    pub fn format_board(&self) {
        // sort the board state values
        let mut board_vec: Vec<_> = self.state.iter().collect();
        board_vec.sort();

        // seperate the columns and rows
        let mut col: Vec<char> = board_vec.iter().map(|pos| pos.0).collect();
        col = col.into_iter().unique().collect();

        let mut row: Vec<u8> = board_vec.iter().map(|pos| pos.1).collect();
        row = row.into_iter().unique().collect();

        // Find the dimensions m x n of the matrix
        let nsize: usize = row.len();
        let msize: usize = col.len();

        let mut matrix: Vec<Vec<(char, u8)>> = vec![vec![('_', 0); msize]; nsize];

        #[allow(clippy::needless_range_loop, clippy::explicit_counter_loop)]
        for c in 0..msize {
            for r in 0..nsize {
                if !board_vec.is_empty() && board_vec[0].1 == (r as u8 + 1) {
                    matrix[r][c] = (board_vec[0].0, board_vec[0].1);
                    board_vec.remove(0);
                }
            }
        }

        // Display the board as a vec-matrix
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

/// This code snippet is implementing the `From` trait for converting a tuple of two `String` values
/// into a `Position` struct.
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
    /// The function `new` initializes a new `Board` struct with an empty `state` HashSet and `player_move`
    /// set to `None`.
    ///
    /// Returns:
    ///
    /// A new instance of the `Board` struct is being returned. The `state` field is initialized with an
    /// empty `HashSet`, and the `player_move` field is set to `None`.
    fn new() -> Board {
        Board {
            state: HashSet::new(),
            player_move: None,
        }
    }

    // m = row, n = col
    #[allow(clippy::unused_enumerate_index)]
    /// The function `default_state` initializes the game board with pieces in their starting positions.
    ///
    /// Arguments:
    ///
    /// * `size`: The `size` parameter in the `default_state` function represents the dimensions of the
    /// board. It is a tuple containing the number of rows and columns in the board.
    fn default_state(&mut self, size: BoardSize) {
        let m: usize = size.0 as usize;
        let n: usize = size.1 as usize;

        for row in 1..=m {
            for (_col, alpha) in COLMS.iter().take(n).enumerate() {
                self.state.insert(Position(
                    alpha.to_string().chars().next().unwrap(),
                    row as u8,
                ));
            }
        }

        Board::format_board(self);
    }
}
