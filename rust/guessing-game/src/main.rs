use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Guess a number!");
                continue;
            },
        };

        println!("Your guess was {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
