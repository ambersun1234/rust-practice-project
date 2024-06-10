use std::io;
use std::io::Write;

fn fibonacci(a: u32, b: u32, num: u32) -> u32 {
    if num == 0 {
        return a;
    }

    return fibonacci(b, a + b, num - 1);
}

fn main() {
    print!("Enter the number: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let num: u32 = input.trim().parse().expect("Please type a number!");
    let answer = fibonacci(0, 1, num);
    println!("The fibonacci of {num} is {answer}")
}
