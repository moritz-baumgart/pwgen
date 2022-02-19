use dialoguer::{theme::ColorfulTheme, Input};


struct PasswordComponent {
    prompt: String,
    characters: Vec<char>,
    default: Option<bool>
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


    let test1 = prompt_yes_or_no(&PasswordComponent {
        prompt: "Test 1 ".to_string(),
        characters: vec!['a', 'b', 'c'],
        default: None                                           // no default
    }, None, None);

    let test2 = prompt_yes_or_no(&PasswordComponent {
        prompt: "Test 2".to_string(),
        characters: vec!['1', '2', '3'],
        default: Some(true)                                           // yes as default
    }, None, None);
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
