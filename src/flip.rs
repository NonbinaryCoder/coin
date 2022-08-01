use colored::Colorize;
use rand::{prelude::ThreadRng, Rng};

pub fn flip_once(is_tty: bool) {
    if is_tty {
        match rand::random() {
            true => println!("{}", "It's heads!".bright_blue()),
            false => println!("{}", "It's tails!".red()),
        }
    } else {
        match rand::random() {
            true => println!("Heads"),
            false => println!("Tails"),
        }
    }
}

pub fn flip_with_gen(is_tty: bool, gen: &mut ThreadRng) {
    if is_tty {
        match gen.gen() {
            true => print!("{}", "Heads ".bright_blue()),
            false => print!("{}", "Tails ".red()),
        }
    } else {
        match gen.gen() {
            true => print!("Heads "),
            false => print!("Tails "),
        }
    }
}

pub fn flip_many(is_tty: bool, count: u32, gen: &mut ThreadRng) {
    for _ in 0..count {
        flip_with_gen(is_tty, gen);
    }
}
