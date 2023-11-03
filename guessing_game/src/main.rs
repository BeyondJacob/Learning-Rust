// 'use' means to bring a name into scope a library 
// that isn't in the prelude. This case, we're bringing
// io library into scope, which gives input/output functionality.
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // 'let' is used to create a variable.
    // 'mut' means the variable is mutable. If it wasn't
    // mutable, it would be immutable (like a constant).
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}