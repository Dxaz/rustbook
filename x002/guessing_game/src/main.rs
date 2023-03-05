use rand::Rng;
use std::cmp::Ordering;
use std::io;

// if __name__ == "__main__":
fn main() -> io::Result<()> {
    // print("Guess the number!")
    println!("Welcome to Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // print("Please input your guess: ")
        println!("Please input your guess :");

        // guess = str("")
        let mut guess = String::new();

        // guess = str(input())
        io::stdin()
            // `&mut` means this is a mutable reference to the object `guess. references are immutable by default.
            // and since this is passed into io::stdin().read_line() method as an arg it will store the output
            // into guess which is why it must be a mut variable.
            .read_line(&mut guess)
            .expect("How did you even get here?!?! What are you doing!?");

        // guess = int(guess.strip())
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // print(f"You guessed: {guess}")
        //    println!("You guessed: {guess}");

        // print("You guessed: {}").format(guess)
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number is too low!"),
            Ordering::Greater => println!("Number is too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    Ok(())
}
