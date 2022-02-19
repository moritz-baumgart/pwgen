use dialoguer::{theme::ColorfulTheme, Input};

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


    let lowercase_alpha = prompt_yes_or_no("Include a-z?", None, None, None);           // no default
    let test = prompt_yes_or_no("Include a-z?", None, None, Some(""));     // Default
}

fn prompt_yes_or_no(prompt: &str, positive: Option<Vec<&str>>, negative: Option<Vec<&str>>, default: Option<&str>) -> bool {

    let positive = positive.unwrap_or(vec!["yes", "y"]);
    let negative = negative.unwrap_or(vec!["no", "n"]);

    let theme = &ColorfulTheme::default();
    let mut input = Input::with_theme(theme);
    input.with_prompt(format!("{} [y/n]", prompt));
    input.validate_with(|input: &String| -> Result<(), &str> {
        let i = input.to_lowercase();
        if positive.contains(&i.as_str()) || negative.contains(&i.as_str()) {
            Ok(())
        } else {
            Err("Please enter \"y\" or \"n\"!")
        }
    });

    if let Some(default_value) = default {
        input.default(default_value.to_string());
    }

    let answer: String = input.interact_text().unwrap();

    if positive.contains(&answer.as_str()) {
        true
    } else if negative.contains(&answer.as_str()) {
        false
    } else {
        // This case should never be reached since 'answer' is checked beforehand if it is in 'positive' or 'negative', but rust complains if not included
        panic!("Unexpected answer!") 
    }

} 
