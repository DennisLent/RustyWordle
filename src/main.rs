use std::path::Path;

mod utils;
use utils::cleaner::clean;
use utils::download::download_raw_dictionary;
use utils::selector::json_to_hashmap;

mod app;
use app::MyEguiApp;

mod game_logic;
mod states;

pub const WORLD_LENGTH: usize = 5;
pub const DICT_UTL: &str = "https://raw.githubusercontent.com/matthewreagan/WebstersEnglishDictionary/master/dictionary.json";

fn main() {
    let clean_dictionary_path = Path::new("src/utils/clean_dictionary.json");
    let raw_dictionary_path = Path::new("src/utils/raw_dictionary.json");

    //check needed JSON files
    match (raw_dictionary_path.exists(), clean_dictionary_path.exists()) {
        //both do not exist so we download the raw file and clean it
        (false, _) => {
            println!("DOWNLOADING DICTIONARY FROM WEB");
            match download_raw_dictionary(DICT_UTL, raw_dictionary_path) {
                Ok(_) => println!("DOWNLOAD COMPLETE"),
                Err(e) => println!("{}", e),
            }

            println!("CLEANINING DICTIONARY");
            match clean(WORLD_LENGTH) {
                Ok(_) => println!("Dictionary cleaned successfully."),
                Err(e) => println!("{}", e),
            }
        }

        //only the raw file exists so we clean it
        (true, false) => {
            println!("CLEANINING DICTIONARY");
            match clean(WORLD_LENGTH) {
                Ok(_) => println!("Dictionary cleaned successfully."),
                Err(e) => println!("{}", e),
            }
        }

        //only the raw file exists so we clean it
        (true, true) => {
            println!("DICTIONARY IN PLACE")
        }
    }

    // Else both dictionaries exist and we can proceed with the game
    // Get the hashmap from the cleaned dictionary and pick a random word & defintion
    let dict = match json_to_hashmap() {
        Ok(dict) => dict,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    MyEguiApp::run(&dict);
}
