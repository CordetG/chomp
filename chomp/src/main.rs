#![allow(unused_imports, unused_variables)]

use chomp_board::*;
use std::env;

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

    let chomp_bar: Board = chomp_board::Game::new(BoardSize(m, n));

    // if &args[1] == "chomp" then index equals (&args[2], &args[3])
}
