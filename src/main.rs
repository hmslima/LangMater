use std::io;
use langmater::*;

fn main() {

    let mut raw_pattern = String::new();

    let stdin = io::stdin();

    println!("Insert the pattern:");

    stdin.read_line(&mut raw_pattern).expect("Couldn't read from stdin");

    let raw_strings = String::from("C: bcdfghjklmnpqrstvxz; V: aeiouäöüé; S: ŭĭ;");

    let pattern = parse_pattern(&raw_pattern, &raw_strings);


    for _n in 1..10 {
        let new_word = String::from(generate_word(&pattern));

        println!("> {}", new_word);
    }

}