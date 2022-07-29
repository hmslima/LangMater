use rand::Rng;

#[derive(Debug)]
pub struct Sound {
    optional: bool,
    is_it_first_optional: bool,
    locked_to_previous_optional: bool,
    sounds: Vec<char>,
}

impl Sound {
    fn clone (&self) -> Sound {
        let new_sound = Sound {
            optional: self.optional.clone(),
            is_it_first_optional: self.is_it_first_optional.clone(),
            locked_to_previous_optional: self.locked_to_previous_optional.clone(),
            sounds: self.sounds.clone(),
        };

        new_sound
    }
}

fn are_the_patterns_the_same (one: &Vec<Sound>, other: &Vec<Sound>) -> bool {

    let mut counter: usize = 0;

    if one.len() != other.len() {
        return false;
    }
    else {
        for _n in one {
            if one[counter].sounds != other[counter].sounds {
                return false;
            }
            counter += 1;
        }
    }
    return true;
}

fn parse_chosen_sound_characters(raw_elements: &String) -> Vec<String> {

    let mut set_of_strings = Vec::<String>::new();
    let mut temp_string = String::new();

    for character in raw_elements.chars() {
        if character.is_uppercase() {
            temp_string.clear();
            temp_string.push(character);
        }
        else if character == ':' || character == ' ' {
            continue;
        }
        else if character == ';' {
            set_of_strings.push(temp_string.clone());
        }
        else {
            temp_string.push(character);
        }
    }

    set_of_strings
}

fn find_the_correct_set_of_characters(reference: char, set_of_strings: &Vec<String>) -> Vec<char> {

    for set_of_chars in set_of_strings {
        if set_of_chars.chars().nth(0) == Some(reference) {
            let mut temp_chars = Vec::<char>::new();
            for character in set_of_chars.chars() {
                if !character.is_uppercase() {
                    temp_chars.push(character);
                }
            }
            return temp_chars;
        }
    }

    return vec!['0'];

}

pub fn parse_pattern(raw_pattern: &String, raw_set_of_strings: &String) -> Vec<Sound> {
    let mut pattern = Vec::<Sound>::new();
    let mut optional_char: bool = false;
    let mut steps_until_next_optional: u32 = 0;

    let raw_elements: Vec<String> = parse_chosen_sound_characters(&raw_set_of_strings);

    let mut is_sound_first_optional: bool;
    let mut does_optional_follow_previous_optional_char: bool;

    for character in raw_pattern.chars() {

        if optional_char {
            steps_until_next_optional += 1;
        }

        if character == '\n' || character == '\r' {
            continue;
        }
        else if character == '(' {
            optional_char = true;
            steps_until_next_optional = 0;
        }
        else if character == ')' {
            optional_char = false;
            steps_until_next_optional = 0;
        }
        else {

            if steps_until_next_optional == 1 {
                is_sound_first_optional = true;
                does_optional_follow_previous_optional_char = false;
            }
            else if steps_until_next_optional > 1 {
                is_sound_first_optional = false;
                does_optional_follow_previous_optional_char = true;
            }
            else {
                is_sound_first_optional = false;
                does_optional_follow_previous_optional_char = false;
            }

            if character.is_uppercase() {
                let new_sound = Sound {
                    optional: optional_char,
                    is_it_first_optional: is_sound_first_optional,
                    locked_to_previous_optional: does_optional_follow_previous_optional_char,
                    sounds: find_the_correct_set_of_characters(character, &raw_elements),
                };
                pattern.push(new_sound);
            }
            else {
                let new_sound = Sound {
                    optional: optional_char,
                    is_it_first_optional: is_sound_first_optional,
                    locked_to_previous_optional: does_optional_follow_previous_optional_char,
                    sounds: vec![character],
                };
                pattern.push(new_sound);
            }
        }
    }

    pattern
}

pub fn parse_antipatterns(raw_antipatterns: &String, raw_set_of_strings: &String) -> Vec<Vec<Sound>> {

    let mut anti_patterns = Vec::<Vec::<Sound>>::new();

    let mut temp_string = String::new();

    for character in raw_antipatterns.chars() {

        if character == ' ' {
            continue;
        }
        else if character == ';' || character == ',' || character == '.' || character == '\n' || character == '\r' {
            anti_patterns.push(parse_pattern(&temp_string, &raw_set_of_strings));
            temp_string.clear();
        }
        else {
            temp_string.push(character);
        }
    }

    anti_patterns
}

pub fn generate_word(pattern: &Vec<Sound>, antipatterns: &Vec<Vec<Sound>>) -> String {

    if pattern.len() == 0 {
        return String::from("");
    }

    let mut rng = rand::thread_rng();

    let mut actual_pattern = Vec::<Sound>::new();

    let mut new_word = String::new();

    let mut previous_optional_character_was_used: bool = false;

    let mut still_is_necessary_find_a_result: bool = true;

    while still_is_necessary_find_a_result {
        for character in pattern {
            let use_character: bool = rng.gen_bool(0.5);
            let number_of_characters = character.sounds.len();
            let which_character = rng.gen_range(0..number_of_characters);
    
            if !character.optional {
                new_word.push(character.sounds[which_character]);
                previous_optional_character_was_used = false;
    
                actual_pattern.push(character.clone());
            }
            else {
                if character.is_it_first_optional {
                    if use_character {
                        new_word.push(character.sounds[which_character]);
                        previous_optional_character_was_used = true;
    
                        actual_pattern.push(character.clone());
                    }
                }
                else if character.locked_to_previous_optional && previous_optional_character_was_used {
                    new_word.push(character.sounds[which_character]);
    
                    actual_pattern.push(character.clone());
                }
                else if character.locked_to_previous_optional && !previous_optional_character_was_used {
                    continue;
                }
                else {
                    if use_character {
                        new_word.push(character.sounds[which_character]);
    
                        actual_pattern.push(character.clone());
                    }
                    previous_optional_character_was_used = false;
                }
            }
        }

        still_is_necessary_find_a_result = false;
    
        for antipattern in antipatterns {
            if are_the_patterns_the_same(&actual_pattern, antipattern) {
                actual_pattern.clear();
                still_is_necessary_find_a_result = true;
                new_word.clear();
            }
        }
    }

    new_word
}