use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Default Number Type is i32

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Returns Number Type of u32

    loop {
        println!("Please input your guess: ");

        let _apples = 5; // Immutable
        let mut guess = String::new(); // Mutable

        io::stdin()
            .read_line(&mut guess) // Mutable Reference. Returns Result Value (Either Ok or Err)
            .expect("Failed to read line"); // Result Instance is an Err Value

        // Shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Match Construct let you Express a Variety of Situations your Code Might Encounter
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
