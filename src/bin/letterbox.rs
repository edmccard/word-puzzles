use std::{env, process};

use word_puzzles::letterbox::Puzzle;
use word_puzzles::load_words;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: letterbox <puzzle>");
        process::exit(1);
    }
    let words = load_words();
    let puzzle = Puzzle::new(args[1].as_bytes());
    let pairs = puzzle.solutions(words);
    for pair in pairs {
        println!(
            "{} - {}",
            str::from_utf8(pair.0).unwrap(),
            str::from_utf8(pair.1).unwrap()
        );
    }
}
