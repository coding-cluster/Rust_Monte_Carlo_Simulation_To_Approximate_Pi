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
    
    let n: usize = match n.trim().parse() {
        Ok(num) => {
            println!("{}", "Successful parsing".green());
            num
        },
        
        Err(_) => {
            println!("{}", "Failed to read input".red());
            return;
        }
    };
    
    let mut inside: usize = 0;
    let mut rng = rand::rng();
    let bar_width = 50;
    
    for i in 1..=n {
        let x: f32 = rng.random_range(-1.0..=1.0);
        let y: f32 = rng.random_range(-1.0..=1.0);
        
        if i % (n / bar_width) == 0 {
            let progress = i as f32 / n as f32;
            let filled = (progress * bar_width as f32) as usize;
            
            let bar: String = format!(
                "\r[{}{}] {:>3}%",
                "#".repeat(filled),
                ".".repeat(bar_width - filled),
                (progress * 100.0) as u32,
            );
            
            print!("{}", bar);
            io::stdout().flush().unwrap();
        }
        
        if x*x + y*y <= 1.0 {
            inside += 1;
        }
    }
    
    let pi: f32 = 4.0 * (inside as f32 / n as f32);
    println!("\nThe approximation of pi is: {}", pi);
}


