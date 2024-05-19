#![allow(dead_code)]
use chomp_board::*;

use itertools::{max, Itertools};
use std::collections::HashSet;

/*
winning-move(posn):
    for each remaining row r
        for each remaining column c in r
            if r = 0 and c = 0
                continue
            p ← copy of posn
            chomp r, c from p
            m ← winning-move(p)
            if no winning move is returned
                return the move r, c
   return winning move
*/

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
        let mut p = posn.clone();
        chomp(&mut p, pos.clone());
        let m = winning_move(&p);
        if m.is_none() {
            return Some(pos);
        }
    }
    None
}

fn chomp(posn: &mut HashSet<Position>, pos: Position) {
    let start_row = pos.1;
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
