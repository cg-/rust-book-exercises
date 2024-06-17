use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to my guessing game. You have 3 guesses.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The number is: {}", secret_number);

    for i in 1..=3{
        println!("Please input guess {}.", i);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("error reading line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big."),
            Ordering::Less => println!("Too small."),
        }
    }



}
