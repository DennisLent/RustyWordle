use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Enum for the letters in the wordle game.
/// A letter can be:
/// - Correct: if present and in the right place.
/// - Present: if present but not in the right place.
/// - Wrong: if not present
pub enum LetterState {
    Correct,
    Present,
    Wrong,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
/// Enum used for showing differnt messages based on the game state
/// - CorrectGuess: the guess was accepted, but it was not the right word
/// - WrongGuess: the guess was not accepted
/// - Won: the guess was the right word and the game is won
/// - Lost: the game is lost
pub enum GameState{
    CorrectGuess,
    WrongGuess,
    Won,
    Lost,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::CorrectGuess => write!(f, "The guess was accepted"),
            GameState::WrongGuess => write!(f, "The guess was not accepted"),
            GameState::Won => write!(f, "You won!"),
            GameState::Lost => write!(f, "You lost :("),
        }
    }
}