use std::io;

fn main() {
    println!("Guess the number Game!");

    println!("Enter the number : ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed number : {}", guess);
}