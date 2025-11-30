use std::io::{self, Write};
use rand::prelude::*;
use colored::Colorize;

fn main() {
    print!("How many points do you want to generate? ");
    io::stdout().flush().unwrap();
    
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    
    let n: i32 = match n.trim().parse() {
        Ok(num) => {
            println!("{}", "Successful parsing".green());
            num
        },
        
        Err(_) => {
            println!("{}", "Failed to read input".red());
            return;
        }
    };
    
    let mut inside: i32 = 0;
    
    for i in 1..=n {
        let x: f32 = rand::rng().random_range(-1.0..=1.0);
        let y: f32 = rand::rng().random_range(-1.0..=1.0);
        
        if x*x + y*y <= 1.0 {
            inside += 1;
        }
    }
    
    let pi: f32 = 4.0 * (inside as f32 / n as f32);
    println!("The approximation of pi is: {}", pi);
}


