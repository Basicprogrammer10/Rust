use std::io::{stdin,stdout,Write};

fn main() {
    let string = get_user_input("Factorial: ");
    println!("You typed: {}",string);
    let fact = factorial(&string.parse::<u32>().unwrap());
    println!("Output: {}", pretty_print_int(fact as isize));
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

fn factorial(input:&u32) -> i32 {
    let mut output = 1;
    for i in 1..(input+1) {
        output *= i;
    }

    return output as i32;
}

fn pretty_print_int(i: isize) -> String{
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    return s;
}