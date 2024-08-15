use crate::utils::errors::DictionaryError;
use rayon::prelude::*;
use serde_json::to_writer_pretty;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

/// Helper function to create a new "cleaned" JSON dictionary that only contains n-letter words.
/// For the basic Wordle game, this is going to be 5-letter words; however, I kept it flexible in case someone wants to play with longer words.
/// This function returns a Result, either Ok when the dictionary was cleaned successfully or Err in case issues arise.
pub fn clean(word_length: usize) -> Result<(), DictionaryError> {
    // raw_dictionary should exist at this path and the file should be opened
    let file = match File::open("src/utils/raw_dictionary.json") {
        Ok(f) => f,
        Err(_) => {
            return Err(DictionaryError::PathError);
        }
    };

    // create a reader and deserialize the JSON dictionary into a hashmap
    let reader = BufReader::new(file);
    let dictionary: HashMap<String, String> = match serde_json::from_reader(reader) {
        Ok(dict) => dict,
        Err(_) => {
            return Err(DictionaryError::ParseError);
        }
    };

    // filter the hashmap for words of the specified length in parallel using rayon
    let filtered_words: BTreeMap<String, String> = dictionary
        .into_par_iter()
        .filter(|(word, _)| word.len() <= word_length && !word.contains("-"))
        .collect::<BTreeMap<_, _>>(); // Collect into a BTreeMap to maintain order

    // create the output file
    let output_file = match File::create("src/utils/clean_dictionary.json") {
        Ok(f) => f,
        Err(_) => {
            return Err(DictionaryError::PathError);
        }
    };

    // Create a buffered writer and write the filtered dictionary to the output file
    let writer = BufWriter::new(output_file);
    match to_writer_pretty(writer, &filtered_words) {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(DictionaryError::ParseError);
        }
    }
}
