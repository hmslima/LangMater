use std::io;
use langmater::*;

fn main() {

    let mut raw_pattern = String::new();
    let mut raw_antipatterns = String::new();

    let stdin = io::stdin();

    println!("Insert the pattern:");
    stdin.read_line(&mut raw_pattern).expect("Couldn't read from stdin");

    println!("Insert the antipatterns:");
    stdin.read_line(&mut raw_antipatterns).expect("Couldn't read from stdin");

    let raw_strings = String::from("C: bcdfghjklmnpqrstvxz; V: aeiouäöüé; S: ŭĭ;");

    let pattern = parse_pattern(&raw_pattern, &raw_strings);
    let antipatterns = parse_antipatterns(&raw_antipatterns, &raw_strings);

    for _n in 1..30 {
        let new_word = String::from(generate_word(&pattern, &antipatterns));

        println!("> {}", new_word);
    }

}