use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let guess = input.lines().next().expect("Failed to read line").trim();

        let number: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.eq("exit") {
                    println!("Bye!");
                    break;
                }
                println!("{} is an invalid input!",guess);
                continue;
            },
        };

        println!("You guessed: {guess}");

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
