use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process;

#[derive(Parser)]
#[command(name = "salut")]
#[command(about = "A multilingual greeting CLI tool with profanity levels")]
#[command(version = "0.1.0")]
struct Args {
    #[arg(short, long, help = "Name to greet")]
    name: String,

    #[arg(short, long, default_value = "en", help = "Language (en, fr, es, de)")]
    language: String,

    #[arg(short, long, default_value = "0", help = "Profanity level (0-2)")]
    profanity: u8,
}

#[derive(Serialize, Deserialize)]
struct GreetingData {
    greetings: HashMap<String, HashMap<String, String>>,
}

impl GreetingData {
    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let greetings_json = include_str!("../greetings.json");
        let data: GreetingData = serde_json::from_str(greetings_json)?;
        Ok(data)
    }

    fn get_greeting(&self, language: &str, profanity: u8, name: &str) -> Result<String, String> {
        let lang_greetings = self.greetings.get(language).ok_or_else(|| {
            format!(
                "Language '{}' not supported. Available: en, fr, es, de",
                language
            )
        })?;

        let profanity_str = profanity.to_string();
        let template = lang_greetings
            .get(&profanity_str)
            .ok_or_else(|| format!("Profanity level '{}' not supported. Use 0-2", profanity))?;

        Ok(template.replace("{name}", name))
    }
}

fn main() {
    let args = Args::parse();

    if args.profanity > 2 {
        eprintln!("Error: Profanity level must be 0-2");
        process::exit(1);
    }

    let greeting_data = match GreetingData::load() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading greetings data: {}", e);
            process::exit(1);
        }
    };

    match greeting_data.get_greeting(&args.language, args.profanity, &args.name) {
        Ok(greeting) => println!("{}", greeting),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_data_load() {
        let data = GreetingData::load().unwrap();
        assert!(data.greetings.contains_key("en"));
        assert!(data.greetings.contains_key("fr"));
        assert!(data.greetings.contains_key("es"));
        assert!(data.greetings.contains_key("de"));
    }

    #[test]
    fn test_get_greeting() {
        let data = GreetingData::load().unwrap();

        let greeting = data.get_greeting("en", 0, "Alice").unwrap();
        assert_eq!(greeting, "Hello Alice!");

        let greeting = data.get_greeting("fr", 1, "Bob").unwrap();
        assert_eq!(greeting, "Salut Bob! Putain!");

        let greeting = data.get_greeting("es", 2, "Carol").unwrap();
        assert_eq!(greeting, "Hola Carol y que te jodan!");
    }

    #[test]
    fn test_invalid_language() {
        let data = GreetingData::load().unwrap();
        let result = data.get_greeting("it", 0, "Alice");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_profanity() {
        let data = GreetingData::load().unwrap();
        let result = data.get_greeting("en", 3, "Alice");
        assert!(result.is_err());
    }
}
