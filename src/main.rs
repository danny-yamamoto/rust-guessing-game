use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();  // mut stands for mutable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    print!("You guessed: {}", guess);
}
