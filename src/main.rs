use std::io;
use std::cmp::Ordering;

use rand::Rng;


fn main() {
    println!("Guess the number!");

    let mut rng = rand::rng();

    let secret_number = rng.random_range(1..=100);
    loop {
            
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess).expect("Failed to read line");
    
        // convert the type of guess from string to integer
        // handle the error to avoid the crash
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; 
    
        println!("You guessed: {guess}");
    
        // compare the guess with secret number
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // when the guess is matched, get out of the loop
                break; 
    
            },
        }
    }

}
