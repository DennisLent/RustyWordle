use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Result};

/// Helper function that reads the cleaned dictionary and returns the Hashmap to it
pub fn json_to_hashmap() -> Result<HashMap<String, String>> {
    // open the cleaned dictionary file
    let file = match File::open("src/utils/clean_dictionary.json") {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening clean_dictionary.json: {}", e);
            return Err(e);
        }
    };

    // create a reader for the file and deserialize JSON into hashmap
    let reader = BufReader::new(file);
    let dictionary: HashMap<String, String> = match serde_json::from_reader(reader) {
        Ok(dict) => dict,
        Err(e) => {
            println!("Error reading or parsing clean_dictionary.json: {}", e);
            return Err(e.into());
        }
    };

    Ok(dictionary)
}

/// Helper funtion to choose a random word and definition from the cleaned dictionary hashmap.
/// This function returns a Result(word, definiton).
pub fn pick_random_word(dictionary: &HashMap<String, String>, word_length: usize) -> Result<(String, String)> {
    // convert the dictionary keys (words) into a vector and pick a random word
    let words: Vec<&String> = dictionary.keys().filter(|&word| word.len() == word_length).collect();
    let word = match words.choose(&mut rand::thread_rng()) {
        Some(&w) => w,
        None => {
            let err_msg = "The dictionary is empty.";
            eprintln!("{}", err_msg);
            return Err(Error::new(ErrorKind::NotFound, err_msg));
        }
    };

    // get definition for the chosen word
    let definition = match dictionary.get(word) {
        Some(def) => def.clone(),
        None => {
            let err_msg = "The selected word does not have a definition.";
            eprintln!("{}", err_msg);
            return Err(Error::new(ErrorKind::NotFound, err_msg));
        }
    };

    // return (word, definiton) tuple
    Ok((word.clone(), definition))
}
