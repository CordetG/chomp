# Chomp

Cordet Gula  
Rust-Lang Fall '24  
Hw3 - chomp  
Professor: Bart Massey

## About

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

```zsh
$ cargo new --lib chomp-lib
    Created library `chomp-lib` package

$ cargo new chomp-bin
    Created binary (application) `chomp-bin` package

chomp/chomp-lib$ cargo build
    Compiling chomp-lib v0.1.0 <path>
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s

chomp/chomp-bin$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 1.91s

$ cargo add impl
```

I followed the program guidelines from the rust-programming class:

```zsh
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

## Run

```zsh
# "--" indicates commands for the program rather than cargo.
# The first command (Vec index 0) is the binary. 
# The following commands are the user input.

$ cargo run -- comm1 comm2
```

Reference for I/O: https://doc.rust-lang.org/stable/book/ch12-01-accepting-command-line-arguments.html

<!-- Update EVerything below -->

## WIP
>
> However, I created my own rust docs.
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

## Resources

The following resources were used in this project:

- [mintlify Doc Writer](https://github.com/mintlify/writer)
- [Rust std](https://doc.rust-lang.org/std/index.html)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Ascii Art Generator](https://www.asciiart.eu/text-to-ascii-art)

## Starting

```zsh
# Clone this project
$ git clone https://github.com/CordetG/rust_programming/chomp

# Access
$ cd chomp/chomp-bin

# Run the project
//$ cargo run chomp

# Test the project
$ cargo test
```

## License

This project is under license from MIT. For more details, see the [LICENSE](../LICENSE) file.

Made by [Cordet Gula](https://github.com/CordetG)

&#xa0;

<a href="#top">Back to top</a>
