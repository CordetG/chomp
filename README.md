# Chomp

Cordet Gula  
Rust-Lang Fall '24  
Hw3 - chomp  
Professor: Bart Massey

## About

The chomp game is developed using [algabraic notation, as in chess](https://www.chessjournal.com/chess-notation/) -- with alphabet characters as the columns and digits as the rows. 

:exclamation: Description referenced from the Canvas assignment page.

Chomp is a strategy-game played by two players whose main goal is to get the opposing player to eat the *poisoned* chocolate square located in the top-left at location (0,0). The process involves leaving no other option for the other player.

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
  - A board state at `(0,0)` indicates the end of the game - a loss.
  - Any other board state indicates a possible move.
- For each move:
  - Create a new board
  - Perform a move on `p`
  - If not a winning move
    - continue
  - else
    - return the winning move
A player loses when the poisoned square at `(0,0)` is that player's only move - the last square on the board.

A move is represented by `chomp i j`. The total possible on non-losing moves is `(m * n) - 1`.

## Features

- [ ] Takes command line input
- [ ] Passed all tests implemented
- [ ] Formatted with `cargo fmt`
- [ ] Passed `cargo clippy`

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

```sh
Chomp is played on a terminal interface.

$ What is the board size?: <m> <n>

Repeat
$ <Current board state>
$ Input a move: <m> <n>

Strategy
$ <Entered move is a winning move>
$ For non-winning moves: <Enter the furthest-right piece in the lowermost, nonempty row>
This will stall a losing move - allowing more time for the other player to make a mistake.
```

### How it went 

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

## Run

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

<!-- Update EVerything below -->
WIP
<!---
In the book, I also utilized ```cargo fix``` to fix compilation errors more efficiently. Even though it was generally minor changes, they were still changes I might have missed otherwise.

```text
Checking toy_rsa_lib v0.1.0
    Fixed examples/toyrsa.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 1.18s
```

>**How it Went**\
Overall, it went pretty well. The point in which I got stuck for an insurmountable amount of time was when I went to run the tests. My ```test_decrypt``` just would not pass. I tried changing the logic, rewriting it a different way, ran through the documents and the algorithm over and over. I could not seem to overcome it.

>However, eventually I did prevail, and I misunderstood lambda function how that was affecting the private keys. This made sense that the decrypt() function was affected because the private keys are used for decryption and the public key for the encryption.

>The error I came across, was misunderstanding that I needed to implement p-1 and q-1 when working with the private keys. I was originally passing in p and q. This miniscule oversight was the largest hurtle I had come across.

>Other than that, the assignment was pretty fun. I continue to learn new things in rust, and as far as this assignment goes, I have a better understanding of libraries and cryptography.

>**Testing**\
I utilized the tests from Canvas including `cargo run 2 20 17` where `2^20 modulo 17 = 16`. It was difficult to test the error-handling though because the program will panic if there is faulty input.

>Testing was implemented with `cargo test`. The modexp results for both tests passed:

```zsh
$ cargo clippy --all
    Checking toy-rsa v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.63s
```

Additionally, `cargo clippy` returned no errors.
-->

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
