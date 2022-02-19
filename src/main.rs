use dialoguer::{theme::ColorfulTheme, Input};
use std::{collections::HashMap, cmp::Eq};


#[derive(Debug)]
struct PasswordComponent {
    prompt: String,
    characters: Vec<char>,
    default: Option<bool>,
    include: bool
}

impl PasswordComponent {
    fn new(prompt: String, characters: Vec<char>, default: Option<bool>,) -> PasswordComponent {
        PasswordComponent {
            prompt,
            characters,
            default,
            include: false
        }
    }
}


fn main() {
    let pw_count: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("How many passwords do you want to generate?")
        .default("1".to_string())
        .validate_with(|input: &String| -> Result<(), &str> {
            let mut numeric = true;
            for c in input.chars() {
                if !c.is_numeric() {
                    numeric = false;
                    break;
                }
            }
            if numeric {
                Ok(())
            } else {
                Err("Please enter a number!")
            }
        })
        .interact_text()
        .unwrap();


    let mut components = [
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
            "!”#$%&()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect(),
            Some(true)  
        ),
        PasswordComponent::new(
            "Include symbols (e.g. @#$%)?".to_string(),
            "!”#$%&()*+,-./:;<=>?@[\\]^_`{|}~".chars().collect(),
            Some(false)
        )
    ];

    for component in &mut components {
        component.include = prompt_yes_or_no(&component, None, None);
    }
}

fn prompt_yes_or_no(password_component: &PasswordComponent, positive: Option<Vec<&str>>, negative: Option<Vec<&str>>) -> bool {

    let positive = positive.unwrap_or(vec!["yes", "y"]);
    let negative = negative.unwrap_or(vec!["no", "n"]);

    let theme = &ColorfulTheme::default();
    let mut input = Input::with_theme(theme);
    input.with_prompt(format!("{} [y/n]", password_component.prompt));
    input.validate_with(|input: &String| -> Result<(), &str> {
        let i = input.to_lowercase();
        if positive.contains(&i.as_str()) || negative.contains(&i.as_str()) {
            Ok(())
        } else {
            Err("Please enter \"y\" or \"n\"!")
        }
    });

    if let Some(default_value) = password_component.default {
        if default_value {
            input.default("Yes".to_string());
        } else {
            input.default("No".to_string());
        }
    }

    let answer: String = input.interact_text().unwrap().to_lowercase();

    if positive.contains(&answer.as_str()) {
        true
    } else if negative.contains(&answer.as_str()) {
        false
    } else {
        // This case should never be reached since 'answer' is checked beforehand if it is in 'positive' or 'negative', but rust complains if not included
        panic!("Unexpected answer!") 
    }

} 
