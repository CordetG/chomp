# Chomp

Cordet Gula  
Rust-Lang Fall '24  
Hw3 - chomp  
Professor: Bart Massey

## About

The chomp game is developed using [algabraic notation, as in chess](https://www.chessjournal.com/chess-notation/) -- with alphabet characters as the columns and digits as the rows. 

:exclamation: Description referenced from the Canvas assignment page.

Chomp is a strategy-game played by two players whose main goal is to get the opposing player to eat the *poisoned* chocolate square located in the top-left at location (0,0) -- or (a, 1) in my design. The process involves leaving no other option for the other player.

Using a recursive algorithm to play the game, This Chomp crate implements an AI agent that plays against a human player.

The **negamax algorithm** of Chomp is presented as follows:

```text
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
```

---

### Chomp Library

The Chomp library implements a state space for the game board represented by `struct Board` using a `HashSet` of tuples `(i,j)`. The purpose of using a HashSet over an array is the improvement in efficiency and mutability of the board's elements.

The following are supported operations for the `Board`:

- Support operations via `impl`
- The user will be able to determine the size of the board `n x m` within a defined limit.
- Print a graphical representation of the board state.
- Support a **`~chomp~`** which removes all squares below &darr; and to the right &rarr; of the chomped square.
- Return a winning move - if there is a winning move.
- `Clone` the new board state.

---

### Chomp Binary Application

The `main.rs` is the binary application file that implements the AI player. That opposes the human {user} player.

Actions:

- Check the board state
  - A board state at `(a,1)` indicates the end of the game - a loss.
  - Any other board state indicates a possible move.
- For each move:
  - Create a new board
  - Perform a move on `p`
  - If not a winning move
    - continue
  - else
    - return the winning move

A player loses when the poisoned square at `(a, 1)` is that player's only move - the last square on the board.

A move is represented by `chomp i j`. The total possible on non-losing moves is `(m * n) - 1`.

## How to Play

To begin run `──> cargo run -- <number of rows> <number of columns>`.  

Example: `──> cargo run -- 4 5` which creates a 4 x 5 board and the starting output will look like this:

```sh

+===========+
|           |
|+-+-+-+-+-+|
||C|h|o|m|p||
|+-+-+-+-+-+|
|           |
+===========+

Rows: 4 x Columns: 5
Columns: ['a', 'b', 'c', 'd', 'e']
Rows: [1, 2, 3, 4]

 (a, 1) is poisoned! 

[('a', 1), ('b', 1), ('c', 1), ('d', 1), ('e', 1)]
[('a', 2), ('b', 2), ('c', 2), ('d', 2), ('e', 2)]
[('a', 3), ('b', 3), ('c', 3), ('d', 3), ('e', 3)]
[('a', 4), ('b', 4), ('c', 4), ('d', 4), ('e', 4)]
```

## <!-- Then for each turn -->

:exclamation: For each turn: ...TODO

## Features

- [X] Takes command line input
- [X] Passed all tests implemented
- [X] Formatted with `cargo fmt`
- [X] Passed `cargo clippy`

## Methods

**Process**

```sh
$ cargo new --lib chomp-board
    Created library `chomp-board` package

$ cargo new chomp
    Created binary (application) `chomp` package

chomp/chomp-board$ cargo build
    Compiling chomp-board v0.1.0 <path>
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s

chomp/chomp$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 1.91s
```

I followed the program guidelines from the rust-programming class:

```text
# Chomp is played on a terminal interface.

Strategy
$ <Entered move is a winning move>
$ For non-winning moves: <Enter the furthest-right piece in the lowermost, nonempty row>
This will stall a losing move - allowing more time for the other player to make a mistake.
```

### How it went 

#### Design

- **Dispay Board vs. board in memory**:
    </br>
    > I used a HashSet for creating, cloning, and changing board states behind-the-scenes. Using a HashSet has nice features and better efficency than say, using an array or Vec. I also noticed the `Difference` function which can return the difference of the current board with a new board -- in other words, the next board state with a move implemented -- and return the resulting board. Which is quite handy.
    >
    > The HashSet is not necessarily user-readable when displaying all of it's contents, unless you just want a list -- but in this context reading a list of positions in a random order is kind of confusing. Maybe a good feature if you want to challenge your cognitive flexability.  
    > ... So, I created a multi-dimensional Vec of the sorted values that is read from the current HashSet so the user can pick their next move more easily. 
    >
    > As the *Master of Over-Thinking* -- as I rewrote this sentence about 5 times -- I could have just removed the complexity of converting a HashSet to a Vec by formatting a display for the HashSet that would be user-readable for the board, but I kept over-analyzing *how* to do that. Displaying a Vec or array is second-nature at this point so that is why I went with the conversion route. 
    >
    > Given more time, I would probably clean up the code so the extra data-structures are not needed.

- **Positions**
    </br>
    > *Would it have been easier to just stick to index positions such as (0,0)?* Absolutely. 
    > In fact, I spent **way** too much time trying to implement it with algabraic notation. *Why?*
    > I like adding complexity; I unconsciously feel the need to spice it up and add extra steps; I have a rebellious trait -- lol -- and IDK....issues. 
    >
    > If I were to go back in time and tell past-me to keep-it-simple, I would. I *objectively* know it would have been faster and I *should* have done it that way to begin with... but I liked the idea.

#### Errors

Notes:
Got error: 'expected identifier, found keyword' --> It took me a second to realize what this meant until I realized the compiler uses the keyword *move* in regard to ownership. The suggestion was to create r#move, but I figured it would be more readable to just change the name of the member in the struct.

:exclamation: To-do: Mention fun ownership/borrowing/lifetime struggles.

Index out of bounds error:

```sh
──> cargo run -- 4 5


+===========+
|           |
|+-+-+-+-+-+|
||C|h|o|m|p||
|+-+-+-+-+-+|
|           |
+===========+

Rows: 4 x Columns: 5
thread 'main' panicked at <path>/chomp-board/src/lib.rs:186:34:
index out of bounds: the len is 11 but the index is 11
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

# Clippy's advice when commenting out '#[allow(needless_loop)]'
consider using an iterator and enumerate(): `(alpha, <item>)`, `clone_rows.iter().enumerate().take(m + 1).skip(1)`

# This was funny
... iterators are lazy and do nothing unless consumed
# Bad Joke: Sounds like clippy has a new philosophy -- eat the lazy <_> that do nothing.
```

Once I fixed the for-loop after realizing I overcomplicated things, go-figure, I tested the printing of the hashset and it did not turn out as expected. I understand that hashsets print in an arbitrary order, so that was not the concern. 

It was the 4 entries when I expected 20, until I realized I had left off trying to get the first dimension functional prior to adding the second dimension. 

So I learned my lesson of leaving todo()s regularly because I will forget, even when I tell myself I will remember -- I never will.

```sh
──> cargo run -- 4 5

...

Rows: 4 x Columns: 5

(c, 2) 
(d, 3) 
(a, 0) 
(b, 1)
```

After adding the 2nd dimension loop and adjusting the output values: 

```sh

──> cargo run -- 4 5

...

Rows: 4 x Columns: 5

(a, 4) 
(c, 4) 
(b, 4) 
(d, 4) 
(e, 3) 
(c, 1) 
(a, 2) 
(b, 3) 
(d, 1) 
(a, 3) 
(d, 2) 
(b, 1) 
(a, 1) 
(c, 3) 
(e, 1) 
(c, 2) 
(e, 4) 
(b, 2) 
(d, 3) 
(e, 2) 

# Yay! it is output as expected -- in an arbitrary order anyway. 
```

Given a 4 x 5 input:

Trying to create a display-board -- for the user to see a board -- was returning 4 copies of each row.
So, I printed the extracted rows and columns:

```sh
Columns: ['a', 'a', 'a', 'a', 'b', 'b', 'b', 'b', 'c', 'c', 'c', 'c', 'd', 'd', 'd', 'd', 'e', 'e', 'e', 'e']
Rows: [1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4]
```

It makes sense because I was trying to display using a matrix -- as a 2D vec so since each letter was split from the Position type, there would be 4. The solution, to remove the duplicate letters and numbers.

Clippy being helpful: `consider using: for (s, r) in row.iter().enumerate()`

#### Things I learned

```sh
# "--" indicates commands for the program rather than cargo.
# The first command (Vec index 0) is the binary. 
# The following commands are the user input.

# When the program is first run, the command line input should be # rows and # of columns for the board
$ cargo run -- <m> <n>

# Following player moves will take in <letter> <number> such as 'b' '4' to indicate move position
```

Reference for I/O: https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html

## Tests

Testing BoardSize fmt implementation:

```sh
$ cargo test

   Doc-tests chomp-board

running 1 test
test src/lib.rs - BoardSize::fmt (line 129) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s
```

## Doc-Comments

```sh
# Open the rust docs
$ cargo doc --open

```

## Resources

The following resources were used in this project:

- [mintlify Doc Writer](https://github.com/mintlify/writer)
- Rust-Analyzer
- [Rust std](https://doc.rust-lang.org/std/index.html)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Ascii Art Generator](https://www.asciiart.eu/text-to-ascii-art)

## License

This project is under license from MIT. For more details, see the [LICENSE](../LICENSE) file.

Made by [Cordet Gula](https://github.com/CordetG)

&#xa0;

<a href="#top">Back to top</a>
