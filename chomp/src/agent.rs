#![allow(dead_code)]
use chomp_board::*;

use itertools::{max, Itertools};
use std::collections::HashSet;

/// The function `winning_move` recursively checks for a winning move in a set of positions in a game.
///
/// Arguments:
///
/// * `posn`: The `posn` parameter is a reference to a `HashSet` containing `Position` objects. Each
/// `Position` object represents a position on a game board, with a tuple `(char, u8)` where the first
/// element is a character representing a column (e.g., 'a') and the second element represents a row.
///
/// Returns:
///
/// The `winning_move` function is returning an `Option` containing a reference to a `Position` if there
/// is a winning move available in the given set of positions. If there is no winning move available, it
/// returns `None`.
fn winning_move(posn: &HashSet<Position>) -> Option<&Position> {
    for pos in posn.iter() {
        if pos.0 == 'a' && pos.1 == 1 {
            continue;
        }
        let mut p: HashSet<Position> = posn.clone();
        chomp(&mut p, pos.clone());
        let m: Option<&Position> = winning_move(&p);
        if m.is_none() {
            return Some(pos);
        }
    }
    None
}

/// The function `chomp` removes positions from a HashSet based on specified criteria.
/// 
/// Arguments:
/// 
/// * `posn`: The `posn` parameter is a mutable HashSet containing positions. Each position is
/// represented as a tuple of two values (x, y) where x represents the column and y represents the row.
/// The function `chomp` takes this HashSet as input and performs certain operations on it based on the
/// provided
/// * `pos`: The `pos` parameter in the `chomp` function represents a position on a grid. It seems to be
/// a tuple containing two elements, where the first element is the column (`pos.0`) and the second
/// element is the row (`pos.1`).
fn chomp(posn: &mut HashSet<Position>, pos: Position) {
    let start_row: u8 = pos.1;
    let row: Vec<_> = posn.iter().map(|pos| pos.1).collect();
    let end_row: u8 = max(row).unwrap();
    let mut col: Vec<_> = posn.iter().map(|pos| pos.0).collect();
    col = col.into_iter().unique().collect();

    col.retain(|&i| i >= pos.0);

    println!("Columns: {:?}", col);

    // Chomp position right and down
    for c in col {
        for i in start_row..=end_row {
            posn.remove(&Position(c, i));
        }
    }
}
