use std::io;
use std::cmp::Ordering;
use rand::RngExt;
use colored::*;

fn main() {

    let secret_number = rand::rng().random_range(1..=100);
    let mut count: u32 = 0;

    loop{
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        count+=1;

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number){
            Ordering::Less=> println!("{}","Too small".red()),
            Ordering::Greater=> println!("{}","Too Large".red()),
            Ordering::Equal=> {
                println!("{}","You win".green());
                break;
            }
        }
    }
    println!("You guessed in {} times", count)
    
}
