use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    let mut rng = thread_rng();
    let number: u32 = rng.gen_range(0..101);

    println!("Guess you a number!");
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");
    
        // Shadow guess
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{}", e);
                println!("Please input a number.");
                continue;
            }
        };
        
        match guess.cmp(&number) {
            Ordering::Less => println!("Too low."),
            Ordering::Equal => {println!("You win!!!!"); break;},
            Ordering::Greater => println!("Too high.")
        }
        println!("You guessed {}", guess);
    }
}
