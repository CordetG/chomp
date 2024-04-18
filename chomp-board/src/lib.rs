#[allow(unused_imports)]
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash)]
struct Position(u8, u8);

struct Board {
    start: Position,
    end: Position,
    player_move: Position,
}

// package def ref
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
