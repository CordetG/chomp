#![allow(unused_imports, unused_variables, unused_assignments)]

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

    let mut winner: bool = false;

    // play game
    while !winner {
        // user turn
        println!("User Turn \nEnter as `chomp <alpha-col> <num-row>`");

        let mut user_turn: String = String::new();
        // take in std input
        io::stdin()
            .read_line(&mut user_turn)
            .expect("Did not recieve user input.");

        // cut out spaces
        user_turn.retain(|c| c != ' ');
        // split off string input user_turn = chomp, col = '<letter>', row = '<num>'
        let mut col: String = user_turn.split_off(5);
        let mut row: String = col.split_off(1);
        row.truncate(1);

        let Position(c, r) = Position::from((col, row));

        agent::chomp(&mut chomp_bar_clone.state, Position(c, r));

        Board::format_board(&chomp_bar_clone);

        println!("\nAgent Turn");

        chomp_bar = chomp_bar_clone.clone();

        let agent_move = agent::winning_move(&chomp_bar.state).unwrap().clone();
        format!("Agent_move: {}", agent_move);

        agent::chomp(&mut chomp_bar.state, agent_move);

        Board::format_board(&chomp_bar);

        if chomp_bar.state.is_empty() {
            winner = true;
        }
    }

    //clearscreen::clear().expect("failed to clear screen");
}
