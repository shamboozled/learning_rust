use std::io;
use rand::{self, Rng};

fn main() {
    // initialize the random number generator
    let mut rng = rand::thread_rng();

    // generate the player cards, dealer cards, and total score
    let mut player_card_1: i32 = rng.gen_range(1..=11);
    let player_card_2: i32 = rng.gen_range(1..=11);

    let mut dealer_card_1 :i32 = rng.gen_range(1..=11);
    let dealer_card_2 :i32 = rng.gen_range(1..=11);

    let mut player_score = player_card_1 + player_card_2;
    let mut dealer_score = dealer_card_1 + dealer_card_2;

    // if both cards are 11, set one equal to 1
    if player_score == 22 {
        player_card_1 = 1;
        player_score = 12;
    }

    if dealer_score == 22 {
        dealer_card_1 = 1;
        dealer_score = 12;
    }

    // show the player their cards and one of the dealer's cards and wait for them to decide to hit or stand
    println!("Welcome to Blackjack! Your cards are {player_card_1} and {player_card_2}. The dealer's first card is {dealer_card_1}.");
    
    // Enter the Blackjack loop that ends when the score exceeds 21 or when the player ends the game
    loop {
        println!("Enter 'h' to draw another card or 's' to end the game.");

        // read the value from the standard input stream and handle potential errors
        let mut guess = String::new();
        io::stdin()
                .read_line(&mut guess)
                .expect("error reading guess");
        
        // remove the extra newline character
        let guess = guess.trim();

        // process the player decision
        if guess == "h" {
            println!("You drew another card!");
            let new_card : i32 = rng.gen_range(1..=11);
            player_score += new_card;
            println!("Your new card is {new_card}. Your score is {player_score}.");
            if player_score > 21 {
                println!("Oh no! You lose.");
                break;
            }
        } else if guess == "s" {
            println!("You ended the game. Your score is {player_score}.");
            println!("The dealer's other card was {dealer_card_2}. The dealer's score is {dealer_score}.");
            if dealer_score > player_score {
                println!("Dealer wins :( womp womp");
            } else if player_score > dealer_score {
                println!("You win! You are so good at Blackjack.");
            } else {
                println!("A tie! What are the odds? Guess we'll have to play again...");
            }
            break;
        } else {
            println!("Invalid input.");
        }
    }
}
