use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    let num = rand::thread_rng().gen_range(1..=101);

    loop {
        print!("Guess the number: ");
        let _ = io::stdout().flush();

        let mut guess_str = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: u32 = guess_str.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");
        match guess.cmp(&num) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win! The number was: {num}");
                break;
            }
        }
    }
}
