use std::io::{stdin,stdout,Write};
use rand::Rng;
use std::process::exit;

static VERSION: &str = "V0.1";
static MIN_NUM: u32 = 1;
static MAX_NUM: u32 = 100;


fn main() {
    let mut rng = rand::thread_rng();
    let mut guesses = 0;
    let mut running = true;

    let random:u32 = rng.gen_range(MIN_NUM..MAX_NUM + 1);

    println!("Welcome to my Guessing Game {}", VERSION);
    println!("Enter a Guess Between {} and {}.", MIN_NUM, MAX_NUM );
    println!("Good Luck :P\n\r");

    while running {
        let guess = get_user_input(">>> ");
        running = check_guess(guess.parse().unwrap(), random, guesses);
        guesses += 1;
    }
}

fn get_user_input(input:&str) -> String {
    let mut s=String::new();
    print!("{}", input);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }

    return s;
}

fn check_guess(guess:u32, random:u32, guesses:u32) -> bool{
    if guess == random {
        println!("  Correct!");
        println!("You won in {} guesses!", guesses + 1);
        return false;
    }
    else if guess > random {
        println!("  Go Lower");
    }
    else if guess < random {
        println!("  Go Higher");
    }

    return true;
}