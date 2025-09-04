fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut guess = String::new();

    loop {
        println!("Please input your guess.");

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }

        guess.clear();
    }
}