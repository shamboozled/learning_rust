use std::io;
use rand::Rng;

fn main() {
    let rand_num: i32 = rand::thread_rng().gen_range(0..100);

    loop {
        let mut guess = String::new();
        
        println!("Enter a guess!");

        io::stdin()
            .read_line(&mut guess)
            .expect("error reading guess");

        let guess: i32 = guess.trim().parse().expect("Invalid guess");
        
        println!("You guessed {guess}");
        
        if guess > rand_num {
            println!("Too large, guess again!");
        } else if guess < rand_num {
            println!("Too small, guess again!");
        } else {
            println!("Correct! Game over.");
            break;
        }
    }
}
