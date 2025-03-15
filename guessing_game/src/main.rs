use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); //

        println!("You guessed : {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Sorry, too small"),
            Ordering::Equal => println!("Congraaats,you guessed !!!"),
            Ordering::Greater => {
                println!("Oopsie, too big");
                break;
            }
        }
    }
}
