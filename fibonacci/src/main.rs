use std::{io, process::exit};

fn main() {
    println!("Fibonacci Program");
    println!(
        "1. Print n fibonacci numbers.\n2. Print the n'th fibonacci number.\n3. Exit\nEnter your choice:"
    );
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number!");
    match choice {
        1 => get_fib(),
        2 => get_nth_fib(),
        3 => exit(0),
        _ => println!("Invalid choice!\nPlease try again."),
    }
}

fn get_fib() {
    println!("Please enter the range (n) to print the numbers upto:");
    let mut range = String::new();
    io::stdin()
        .read_line(&mut range)
        .expect("Failed to read input.");
    let range: u32 = range
        .trim()
        .parse()
        .expect("Please enter a valid number as range!");
    let mut a = 0;
    let mut b = 1;
    if range > 0 {
        println!("Fibonacci numbers upto {range} are:");
        print!("{a}");
    } else {
        println!("Please enter a valid non zero range (1...inf)");
    }
    if range > 1 {
        print!(", {b}");
        let mut i = 2;
        while i < range {
            let c = a + b;
            print!(", {c}");
            a = b;
            b = c;
            i += 1;
        }
        println!("");
    }
}

fn get_nth_fib() {
    println!("Which number (n) in the fibonacci sequence would you like to generate?");
    let mut term = String::new();
    io::stdin()
        .read_line(&mut term)
        .expect("Failed to read input!");
    let term: u32 = term
        .trim()
        .parse()
        .expect("Please enter a valid number as the term! (1...inf)");
    let res: u32 = {
        if term == 1 {
            1
        } else if term == 2 {
            2
        } else {
            let mut i = 2;
            let mut a = 0;
            let mut b = 1;
            let mut c = 0;
            while i < term {
                c = a + b;
                a = b;
                b = c;
                i += 1;
            }
            c
        }
    };
    println!("The {term}th term in fibonacci series is: {res}");
}
