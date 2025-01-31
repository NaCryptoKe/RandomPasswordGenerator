// Import necessary modules from the standard library and external crates
use std::io::{self, Write};  // For input/output operations
use std::cmp;                // For comparison operations
use rand::seq::SliceRandom;  // For random selection from collections
use rand::thread_rng;        // For thread-local random number generator

/// Prompts user with yes/no question and returns boolean response
/// Loops until valid input ('y' or 'n') is provided
fn get_yes_no_input(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();  // Ensure prompt displays immediately

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        // Normalize input: trim whitespace and convert to lowercase
        let choice = choice.trim().to_lowercase();

        if choice == "y" || choice == "n" {
            return choice == "y";  // Return true for 'y', false for 'n'
        }

        println!("Invalid input! Please enter 'y' or 'n'.");
    }
}

/// Gets valid password length from user that meets minimum requirement
/// Handles non-integer inputs and values below minimum length
fn get_valid_password_length(min_length: i32) -> i32 {
    loop {
        let mut input = String::new();
        print!("Enter password length (minimum {}): ", min_length);
        io::stdout().flush().unwrap();  // Ensure prompt displays before input

        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<i32>() {
            Ok(length) => {
                if length >= min_length {
                    return length;
                } else {
                    println!("Error: Password length must be at least {}.", min_length);
                }
            }
            Err(_) => {
                println!("Invalid input! Please enter a whole number.");
            }
        }
    }
}

fn main() {
    // Define character sets as vectors for efficient random access
    let lowercase_chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let uppercase_chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let numbers: Vec<char> = "0123456789".chars().collect();
    let special_chars: Vec<char> = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect();
    
    // Get user preferences for character types
    let include_lowercase = get_yes_no_input("Include lowercase letters? (y/n): ");
    let include_uppercase = get_yes_no_input("Include uppercase letters? (y/n): ");
    let include_numbers = get_yes_no_input("Include numbers? (y/n): ");
    let include_special_chars = get_yes_no_input("Include special characters? (y/n): ");
    
    // Calculate minimum password length based on selected character types
    let selected_types_count = 
        (include_lowercase as i32) + 
        (include_uppercase as i32) + 
        (include_numbers as i32) + 
        (include_special_chars as i32);
        
    // Ensure password can contain at least one of each selected type
    let min_password_length = cmp::max(1, selected_types_count);
    let password_length = get_valid_password_length(min_password_length);
    
    // Build character pool based on user selections
    let mut character_pool: Vec<char> = Vec::new();
    if include_lowercase {
        character_pool.extend(lowercase_chars.iter());
    }
    if include_uppercase {
        character_pool.extend(uppercase_chars.iter());
    }
    if include_numbers {
        character_pool.extend(numbers.iter());
    }
    if include_special_chars {
        character_pool.extend(special_chars.iter());
    }
    
    // Safety check: prevent empty password generation
    if character_pool.is_empty() {
        println!("Error: You must select at least one character type!");
        return;
    }
    
    // Initialize random number generator
    let mut rng = thread_rng();
    
    // Generate base password with random characters from pool
    let mut password: Vec<char> = character_pool
        .choose_multiple(&mut rng, password_length as usize)
        .cloned()
        .collect();
    
    // Ensure at least one character from each selected type
    let mut required_chars = Vec::new();
    if include_lowercase {
        required_chars.push(*lowercase_chars.choose(&mut rng).unwrap());
    }
    if include_uppercase {
        required_chars.push(*uppercase_chars.choose(&mut rng).unwrap());
    }
    if include_numbers {
        required_chars.push(*numbers.choose(&mut rng).unwrap());
    }
    if include_special_chars {
        required_chars.push(*special_chars.choose(&mut rng).unwrap());
    }
    
    // Replace first characters with required ones
    for i in 0..required_chars.len() {
        password[i] = required_chars[i];
    }
    
    // Shuffle password to randomize required characters
    password.shuffle(&mut rng);
    
    // Convert character vector to String and display result
    println!("Generated Password: {}", password.iter().collect::<String>());
}