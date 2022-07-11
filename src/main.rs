use std::io;

// Main function of the app
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();          // Mutable variable to store user input

    io::stdin()
        .read_line(&mut guess)              // Read from keyboard and store it on guess variable (parameter is a pointer)
        .expect("Failed to read line");     // On error throw exception

    println!("You guessed: {guess}");       // Print the result
}