use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing name");
    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Enter the guessing number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error msg");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
