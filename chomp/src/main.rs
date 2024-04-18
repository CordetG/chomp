#[allow(unused_imports)]
use std::collections::HashSet;
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

    let (_m, _n) = (&args[1], &args[2]);

    // if &args[1] == "chomp" then index equals (&args[2], &args[3])
}
