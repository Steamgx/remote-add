# Guessing Game

This project is a simple guessing game implemented in Rust. The player has to guess a randomly generated number within a certain range.

## Project Structure

```
guessing_game/
├── Cargo.toml          # Project manifest and dependencies
├── Cargo.lock          # Locked dependency versions
├── src/
│   └── main.rs         # Main application code
├── target/             # Build artifacts (auto-generated)
├── README.md           # Project documentation
└── .gitignore          # Git ignore rules
```

## Getting Started

To build and run the application, follow these steps:

1. Ensure you have Rust installed on your machine. You can install it from [rust-lang.org](https://www.rust-lang.org/).

2. Clone the repository:

   ```
   git clone https://github.com/microsoft/vscode-remote-try-rust.git
   cd guessing_game
   ```

3. Build the project:

   ```
   cargo build
   ```

4. Run the application:

   ```
   cargo run
   ```

## Gameplay

Once the application is running, it will prompt you to guess a number. You will receive feedback on whether your guess is too high, too low, or correct. Keep guessing until you find the correct number!

## Contributing

Feel free to submit issues or pull requests if you have suggestions or improvements for the project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.