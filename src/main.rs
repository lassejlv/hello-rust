mod helpers;

use helpers::prompt::prompt;
use rand::Rng;

fn main() {
    let guess: i32 = match prompt("Enter a number from 1 - 10 ").parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    let max_number = 10;

    if guess > max_number || guess < 1 {
        println!("Please enter a number between 1 and 10");
        return;
    }

    let random_secret_num = rand::thread_rng().gen_range(1..=10);

    if guess == random_secret_num {
        println!("You guessed correctly!");
    } else {
        println!(
            "You guessed incorrectly! The secret number was: {}",
            random_secret_num
        );
    }
}
