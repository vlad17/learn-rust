// use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn attempt_to_grab_number() -> Option<u32> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read guess");

    return guess.trim().parse().ok();
}

fn main() {
    let secret = 3; //rand::thread_rng().gen_range(1, 101);
    println!("Secret really is {}", secret);

    println!("Now guess the secret");

    loop {
        let guess = match attempt_to_grab_number() {
            Some(n) => n,
            None => {
                // QUESTION
                // How does this branch compile? What's the
                // type of "continue"? Does rust have a "bottom"
                // type like in Haskell or does it ignore types
                // for certain expressions at the language level
                // (like Java with exceptions?).
                println!("failed to parse positive number");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
            Ordering::Greater => println!("Too large!"),
        };
    }
}
