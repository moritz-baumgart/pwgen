use dialoguer::{theme::ColorfulTheme, Input};
use rand::{thread_rng, seq::SliceRandom};

// Represents a 'component' of a password, e.g. uppercase letters, numbers, symbols.
struct PasswordComponent {
    prompt: String,             // Holds the prompt which is used to ask if this component should be part of the password
    characters: Vec<char>,      // Holds the characters of the component.
    default: Option<bool>,      // Tells the prompt function if this component should be in the password by default (true) or not (false), or if there is no default value (None).
}

impl PasswordComponent {
    // Associated function to initialize the struct easily.
    fn new(prompt: String, characters: Vec<char>, default: Option<bool>,) -> PasswordComponent {
        PasswordComponent {
            prompt,
            characters,
            default,
        }
    }
}


fn main() {

    // The prompt_number function is used to prompt for the number of password to be generated and their length. 
    let pw_count = prompt_number("How many passwords do you want to generate?", 1);
    let pw_length = prompt_number("How long do you want your password(s) to be?", 10);

    // The following array holds the components I chose to be available. At the moment this includes lowercase/uppercase letters, numbers (0-9) and symbols like @#$%.
    let components = [
        PasswordComponent::new(
            "Include lowercase alphabet (a-z)?".to_string(),
            "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            Some(true)
        ),
        PasswordComponent::new(
            "Include uppercase alphabet (A-Z)?".to_string(),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            Some(true)
        ),
        PasswordComponent::new(
            "Include numbers (0-9)?".to_string(),
            "0123456789".chars().collect(),
            Some(true)  
        ),
        PasswordComponent::new(
            "Include symbols (e.g. @#$%)?".to_string(),
            "!”#$%&()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect(),
            Some(false)
        )
    ];

    // In the following the user is prompted for each of the components in the array if it should be part of his password.
    // If he gives a positive answer the components characters are added to 'all_chars' to be used later.
    let mut all_chars = Vec::new();

    for component in components {
        if prompt_yes_or_no(&component, None, None) {
            all_chars.extend(component.characters);
        }
    }

    // Notify the user that the following output is/are his password(s).
    // Print singular or plural depending on how many password the user chose. 
    println!();
    if pw_count > 1 {
        println!("Here are your passwords:");
    } else {
        println!("Here is your password:");
    }
    println!();


    // Retrieve the thread local rng to later chose random characters.
    let mut rng = thread_rng();
    
    // Loop 'pw_count' times to generate the appropriate number of passwords.
    for _ in 0..pw_count {
        let mut password = "".to_string();

        // Loop 'pw_length' times and in each iteration add a randomly chosen character from the all_chars vec and push it the the 'password' String.
        for _ in 0..pw_length {
            password.push(*all_chars.choose(&mut rng).unwrap());
        }

        // 'password' now holds a randomly generated password of the desired length, so we print it to the console.
        println!("{}", password);
    }
}


// This function is used to prompt the user for a number using the dialoguer crate.
// It takes a prompt: &str which the user will be presented and a default: u32 which the user can select by just pressing enter without entering anything.
fn prompt_number(prompt: &str, default: u32) -> u32 {
    let number: String = Input::with_theme(&ColorfulTheme::default())   // A new dialoguer Input is created with the provided default theme.
        .with_prompt(prompt)    // Sets the given prompt.
        .default(default.to_string())   // Sets the given default number.
        .validate_with(|input: &String| -> Result<(), &str> { // Validator iterates over characters in user input, if a non numeric one if found it aborts and returns Err.
            let mut numeric = true; 
            for c in input.chars() {
                if !c.is_numeric() {    // Note: Since we are iterating over each character this check is given when the user enters a negative number (because auf the leading '-'). Therefore we can safely use u32.
                    numeric = false;
                    break;
                }
            }
            if numeric {
                Ok(())  // If the whole input is numeric return Ok.
            } else {
                Err("Please enter a number!") // Otherwise return Err with a message to the user. Dialoguer prints this to console. 
            }
        })
        .interact_text()
        .unwrap();
    number.parse::<u32>().unwrap()  // Parse the input String to u32. Will never panic since number is verified if it is positive and numeric
}


// This function prompts the user if a given PasswordComponent should be part of the password.
// Besides the PasswordComponent 2 Options containing a Vec<&str> can be passed to the function to allow alternative terms for an positve/negative answer (e.g. in a diffrent language).
fn prompt_yes_or_no(password_component: &PasswordComponent, positive: Option<Vec<&str>>, negative: Option<Vec<&str>>) -> bool {

    let positive = positive.unwrap_or(vec!["yes", "y"]);    // If no alternative answer terms are given, ovewrite them with default ones.
    let negative = negative.unwrap_or(vec!["no", "n"]);

    // Note: In the following we dont chain the methods of Input like in 'prompt_number' this way we can dynamically set the default value.
    let theme = &ColorfulTheme::default();
    let mut input = Input::with_theme(theme); // A new dialoguer Input is created with the provided default theme.
    input.with_prompt(format!("{} [y/n]", password_component.prompt));
    input.validate_with(|input: &String| -> Result<(), &str> {
        let i = input.to_lowercase();   // Validator checks if the user given answer is part of the terms used for an positive/negative answer. It returns Ok and Err respectively, similar to 'prompt_number'
        if positive.contains(&i.as_str()) || negative.contains(&i.as_str()) {
            Ok(())
        } else {
            Err("Please enter \"y\" or \"n\"!")
        }
    });

    // If a default value is set (true/false), set a default accordingly. Otherwise no default is set.
    if let Some(default_value) = password_component.default {
        if default_value {
            input.default("Yes".to_string());
        } else {
            input.default("No".to_string());
        }
    }

    let answer: String = input.interact_text().unwrap().to_lowercase();

    // Depending an the answer the user gave, true or false is returned. 
    if positive.contains(&answer.as_str()) {
        true
    } else if negative.contains(&answer.as_str()) {
        false
    } else {
        // This case should never be reached since 'answer' is checked beforehand if it is in 'positive' or 'negative', but rust complains if not included
        panic!("Unexpected answer!") 
    }

} 
