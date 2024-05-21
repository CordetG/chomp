#![allow(unused_imports, unused_variables)]

use chomp_board::*;
use std::collections::HashSet;
use std::env;
use std::io;
use std::io::Write;

mod agent;
pub use agent::*;

const TITLE: &str = "
+===========+
|           |
|+-+-+-+-+-+|
||C|h|o|m|p||
|+-+-+-+-+-+|
|           |
+===========+
";

fn main() {
    clearscreen::clear().expect("failed to clear screen");

    println!("{}", TITLE);

    let args: Vec<String> = env::args().collect();

    println!("Rows: {} x Columns: {}", args[1], args[2]);

    let arg_box: Box<Vec<String>> = Box::new(args);
    let game_box: Box<Vec<String>> = arg_box.clone();

    let BoardSize(m, n) = BoardSize::from((game_box[1].to_owned(), game_box[2].to_owned()));

    let mut chomp_bar: Board = chomp_board::Game::new(BoardSize(m, n));
    <chomp_board::Board as chomp_board::Game>::default_state(&mut chomp_bar, BoardSize(m, n));

    let mut chomp_bar_clone: Board = chomp_bar.clone();

    // play game

    // user turn
    println!("User Turn \nEnter as `chomp <alpha-col> <num-row>`");

    let mut user_turn: String = String::new();
    // take in std input
    io::stdin()
        .read_line(&mut user_turn)
        .expect("Did not recieve user input.");
    print!("User Move: {}", user_turn);
    // cut out spaces
    user_turn.retain(|c| c != ' ');
    print!("User Move: {}", user_turn);
    // split off string input user_turn = chomp, col = '<letter>', row = '<num>'
    let mut col: String = user_turn.split_off(5);
    let mut row: String = col.split_off(1);
    row.truncate(1);

    let Position(c, r) = Position::from((col, row));

    agent::chomp(&mut chomp_bar_clone.state, Position(c, r));

    /*let chomped_pieces: HashSet<Position> = chomp_bar
        .state
        .difference(&chomp_bar_clone.state)
        .cloned()
        .collect();

    let new_board = chomp_board::Board::chomped_board(&mut chomp_bar, chomped_pieces);*/
    // Error: function displaying only the original board
    Board::format_board(&chomp_bar_clone);

    //clearscreen::clear().expect("failed to clear screen");
}
