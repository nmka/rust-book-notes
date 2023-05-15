use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the nubmer:");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {

        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess_number}!" );

        match guess_number.cmp(&random_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
