use std::collections::HashMap;
use crate::states::{GameState, LetterState};

#[derive(Debug, Clone)]
/// This struct contains the main logic behind the wordle game.
/// It holds all the previous guesses and the state of each letter.
pub struct WordleGame {
    word: String,
    pub current_guess: [char; 5],
    pub current_row: usize,
    pub guesses: Vec<[LetterState; 5]>,
    pub guesses_letters: Vec<[char; 5]>,
    pub alphabet: HashMap<char, LetterState>,
}

impl WordleGame {
    /// Constructor method for the Wordle game that returns itself
    /// Requires the word that is supposed to be guessed
    pub fn new(word: String) -> Self {
        Self {
            word: word.to_uppercase(),
            current_guess: [' '; 5],
            current_row: 0,
            guesses: Vec::new(),
            guesses_letters: Vec::new(),
            alphabet: ('A'..='Z').map(|c| (c, LetterState::Unknown)).collect(),
        }
    }

    /// Method that checks if the word is contained in the dictionary.
    /// This method also checks for the suffixes of words.
    ///
    /// **Example**: Boats is not in the dictionary, however boat is
    fn is_valid_word(&self, word: String, dictionary: &HashMap<String, String>) -> bool {
        
        if dictionary.contains_key(&word) {
            return true;
        }
        
        let possible_bases = vec![
            word.strip_suffix('s'),
            word.strip_suffix("es"),
            word.strip_suffix("ed"),
            word.strip_suffix("ing"),
        ];

        for base in possible_bases {
            if let Some(base_word) = base {
                if dictionary.contains_key(base_word) {
                    return true;
                }
            }
        }

        return false;
    }

    /// Method to submit a guess
    /// This method requires the dictionary to check if the word submitted is actually a real word
    pub fn submit_guess(
        &mut self,
        dictionary: &HashMap<String, String>,
    ) -> GameState {
        // check if we have exceeded the maximum row count
        if self.current_row >= 5 {
            return GameState::Lost;
        }

        let guess: String = self.current_guess.iter().collect();
        // check if there is an incomplete guess
        if guess.len() != 5 || self.current_guess.contains(&' ') {
            return GameState::WrongGuess;
        }

        // check if the word submitted is a real word by cross-referencing in the dictionary
        if !self.is_valid_word(guess.clone().to_lowercase(), dictionary) {
            return GameState::WrongGuess;
        }

        self.guesses_letters.push(self.current_guess);

        let mut guess_state = [LetterState::Wrong; 5];
        let word_letters: Vec<char> = self.word.chars().collect();

        // check if correct letter in the correct position
        for i in 0..5 {
            if self.current_guess[i] == word_letters[i] {
                guess_state[i] = LetterState::Correct;
            }
        }

        // check if correct letter
        for i in 0..5 {
            if guess_state[i] == LetterState::Correct {
                continue;
            }
            if let Some(_) = word_letters.iter().position(|&c| {
                c == self.current_guess[i]
                    && guess_state[word_letters.iter().position(|&t| t == c).unwrap()]
                        != LetterState::Correct
            }) {
                guess_state[i] = LetterState::Present;
            }
        }

        // store the guesses, update row and reset current guess
        self.guesses.push(guess_state);
        self.current_row += 1;
        self.current_guess = [' '; 5];

        if guess_state
            .iter()
            .all(|&state| state == LetterState::Correct)
        {
            return GameState::Won;
        }
        else{
            return GameState::CorrectGuess;
        }
    }
}
