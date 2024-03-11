# Guess

This is a simple number guessing game implemented in Rust. The program generates a random number between 1 and 1000 (inclusive) and prompts the user to guess it. The user continues to guess until they correctly identify the number.

## Usage

To run the game, ensure you have Rust installed on your system clone this repository, navigate to the project directory and run the program using Cargo:

```
git clone https://github.com/naregderlevonian/guess.git
cd number-guessing-game
cargo run
```

## How to Play

1. Upon starting the game, you will see the game logo.
2. You will be prompted to enter a number between 0 and 1000.
3. After each guess, the program will inform you if your guess was too small or too big.
4. Keep guessing until you correctly identify the generated number.
5. Once you guess correctly, you will see the win screen and the game will end.

## Dependencies

- `rand`: Used for generating random numbers.
- `colorize`: Used for adding colors and styles to console output.

## Calligraphy Credits

The ASCII art logos used in this game are created by [Calligraphy](https://gitlab.com/gregorni/Calligraphy)

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
