// NOTE: this was written on my school laptop, i cannot test the code or see errors or add changes, this could SHOULD work in theory but it is untested.

use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Write}
use colored::Colorize;

pub fn add(abbr: &str, meaning: &str, abbreviations: &mut HashMap<String, Value>) {
    let abbr_lower = abbr.to_lowercase();

    if abbreviations.contains_key(&abbr_lower) {
        let value = &mut abbreviations[&abbr_lower];

        match value {
            // If the value is a string, we convert it into an array with the new meaning
            Value::String(existing_meaning) => {
                *value = Value::Array(vec![
                    json!(existing_meaning),
                    json!(meaning)
                ]);
            }

            // If the value is already an array, we simply add the new meaning
            Value::Array(means) => {
                means.push(json!(meaning));
            }

            _ => {
                // just incase!!!
                println!("{}: Unexpected data type for abbreviation. This is a fault on our end, please report.", "Error".red().bold());
            }
        }
    } else {
        // if it doesn't already exist (which it probably doesn't) make a new entry!!!
        abbreviations.insert(abbr_lower.clone(), json!(meaning));
    }

    let abbr_file_path = format!("{}/abbreviations.json", get_abbr_directory());

    // open the abbreviations.json file
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&abbr_file_path)
        .unwrap();

    // rewrite the file with the updated abbreviations
    let updated_json = serde_json::to_string_pretty(&abbreviations).unwrap();
    file.write_all(updated_json.as_bytes()).unwrap(); // write it

    println!("{}: Abbreviation successfully added/updated.", "Success".green().bold());
}
