use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret numer is {}", secret_number);
    io::stdin().read_line(&mut guess).expect("note the fail");
    println!("you guessed {}", guess);
}
