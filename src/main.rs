extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Ni nambari gani tunafikiria ?");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("Nambari tumeficha ni {}",secret_number);
    loop {
        println!("Andika nambari lako ...");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Sijaelelwa ulichoandika");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Hujandika Nambari!!");
                continue;
            },
        };
        println!("Umeandika: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Ni kubwa!"),
            Ordering::Greater => println!("Ni ndogo!"),
            Ordering::Equal => {
                println!("Umeshinda!");
                break;
            }
        }
    }
}
