use std::io;
use std::io::Write;

fn diamond(num: u32) {
    let _ = io::stdout().flush();

    for _i in 0..num {
        for _ in _i..num {
            print!(" ");
        }
        for _ in 0..(_i+1) {
            print!("*");
        }
        for _ in 0.._i {
            print!("*");
        }
        for _ in _i..num {
            print!(" ");
        }
        println!();
    }
    for _i in 1..num {
        for _ in 0..(_i+1) {
            print!(" ");
        }
        for _ in _i..num {
            print!("*");
        }
        for _ in (_i+1)..num {
            print!("*");
        }
        for _ in 0.._i {
            print!(" ");
        }
        println!();
    }
}

fn main() {
    print!("Enter the number: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let num = input.trim().parse().expect("Please type a number!");

    diamond(num)
}
