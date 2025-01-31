import random
import string

def get_valid_password_length(min_length):
    """
    Prompt the user for a valid password length.
    Ensures the input is a whole number and meets the minimum required length.
    """
    while True:
        try:
            length = int(input(f"Enter password length (minimum {min_length}): ").strip())
            if length >= min_length:
                return length
            print(f"Error: Password length must be at least {min_length} to fit your choices!")
        except ValueError:
            print("Invalid input! Please enter a whole number.")

def get_yes_no_input(prompt):
    """
    Prompt the user for a yes/no input.
    Returns True if 'y', False if 'n'.
    """
    while True:
        choice = input(prompt).strip().lower()
        if choice in ['y', 'n']:
            return choice == 'y'
        print("Invalid input! Please enter 'y' or 'n'.")

# Ask the user which character types to include
include_lowercase = get_yes_no_input("Include lowercase letters? (y/n): ")
include_uppercase = get_yes_no_input("Include uppercase letters? (y/n): ")
include_numbers = get_yes_no_input("Include numbers? (y/n): ")
include_special_chars = get_yes_no_input("Include special characters? (y/n): ")

# Count the number of selected character types
selected_types_count = sum([include_lowercase, include_uppercase, include_numbers, include_special_chars])

# Ensure minimum password length is at least equal to the selected types count
min_password_length = max(1, selected_types_count)
password_length = get_valid_password_length(min_password_length)

# Build the character pool based on user choices
character_pool = ""
if include_lowercase:
    character_pool += string.ascii_lowercase
if include_uppercase:
    character_pool += string.ascii_uppercase
if include_numbers:
    character_pool += string.digits
if include_special_chars:
    character_pool += string.punctuation

# Handle case where no character types are selected
if not character_pool:
    print("Error: You must select at least one character type!")
else:
    # Generate an initial password with random characters
    password = [random.choice(character_pool) for _ in range(password_length)]
    
    # Ensure at least one of each selected character type is included
    required_chars = []
    if include_lowercase:
        required_chars.append(random.choice(string.ascii_lowercase))
    if include_uppercase:
        required_chars.append(random.choice(string.ascii_uppercase))
    if include_numbers:
        required_chars.append(random.choice(string.digits))
    if include_special_chars:
        required_chars.append(random.choice(string.punctuation))

    # Replace the first few characters with required ones to guarantee inclusion
    for i in range(len(required_chars)):
        password[i] = required_chars[i]

    # Shuffle the password to mix required characters randomly
    random.shuffle(password)

    # Convert list to string and display the generated password
    print("Generated Password:", "".join(password))
