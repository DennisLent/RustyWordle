use crate::game_logic::WordleGame;
use crate::states::{GameState, LetterState};
use crate::utils::selector::pick_random_word;
use crate::WORLD_LENGTH;
use eframe::egui;
use egui::{RichText, Vec2, ViewportBuilder};
use std::collections::HashMap;
use std::sync::Arc;

/// Struct to keep all the information of the game together
/// This struct creates the 5 letter grid with 6 attempts and holds the word and its defintion
#[derive(Debug)]
pub struct MyEguiApp {
    grid: [[char; WORLD_LENGTH]; WORLD_LENGTH + 1],
    current_row: usize,
    word: String,
    definition: String,
    game_logic: WordleGame,
    game_state: Option<GameState>,
    last_guessed_word: Option<String>,
    index: usize,
    dictionary: Arc<HashMap<String, String>>,
}

/// Methods for the Gui App
impl MyEguiApp {
    /// Constructor method for the app.
    /// This method takes in the word that is supposed to be guessed and it's defintion
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        word: String,
        definition: String,
        dict: Arc<HashMap<String, String>>,
    ) -> Self {
        Self {
            grid: [[' '; WORLD_LENGTH]; WORLD_LENGTH + 1],
            current_row: 0,
            word: word.clone(),
            definition: definition,
            game_logic: WordleGame::new(word),
            game_state: None,
            index: 0,
            dictionary: dict,
            last_guessed_word: None,
        }
    }

    fn new_game(&mut self) {
        let (new_word, new_definition) =
            match pick_random_word(self.dictionary.as_ref(), WORLD_LENGTH) {
                Ok((word, defintion)) => (word, defintion),
                Err(e) => {
                    println!("Error: {}", e);
                    return;
                }
            };

        self.word = new_word.clone();
        self.definition = new_definition;
        self.game_logic = WordleGame::new(new_word);
        self.grid = [[' '; 5]; 6];
        self.current_row = 0;
        self.index = 0;
        self.game_state = None;
    }

    /// Run method for the app
    /// This method takes in the dictionary and picks a random word to guess
    pub fn run(dict: &HashMap<String, String>) {
        let dictionary_clone = Arc::new(dict.clone());

        let native_options = eframe::NativeOptions {
            viewport: ViewportBuilder {
                min_inner_size: Some(Vec2 { x: 200.0, y: 850.0 }),
                max_inner_size: Some(Vec2 {
                    x: 450.0,
                    y: 1200.0,
                }),
                resizable: Some(false),
                ..Default::default()
            },
            centered: true,
            ..Default::default()
        };

        let (word, definition) = match pick_random_word(&dict, WORLD_LENGTH) {
            Ok((word, definition)) => (word, definition),
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let _ = eframe::run_native(
            "RustyWordle",
            native_options,
            Box::new(|cc| {
                Ok(Box::new(MyEguiApp::new(
                    cc,
                    word,
                    definition,
                    dictionary_clone,
                )))
            }),
        );
    }

    /// Helper function to associate key presses with char and actions
    fn key_to_char(&self, key: &egui::Key) -> Option<char> {
        match key {
            egui::Key::A => Some('A'),
            egui::Key::B => Some('B'),
            egui::Key::C => Some('C'),
            egui::Key::D => Some('D'),
            egui::Key::E => Some('E'),
            egui::Key::F => Some('F'),
            egui::Key::G => Some('G'),
            egui::Key::H => Some('H'),
            egui::Key::I => Some('I'),
            egui::Key::J => Some('J'),
            egui::Key::K => Some('K'),
            egui::Key::L => Some('L'),
            egui::Key::M => Some('M'),
            egui::Key::N => Some('N'),
            egui::Key::O => Some('O'),
            egui::Key::P => Some('P'),
            egui::Key::Q => Some('Q'),
            egui::Key::R => Some('R'),
            egui::Key::S => Some('S'),
            egui::Key::T => Some('T'),
            egui::Key::U => Some('U'),
            egui::Key::V => Some('V'),
            egui::Key::W => Some('W'),
            egui::Key::X => Some('X'),
            egui::Key::Y => Some('Y'),
            egui::Key::Z => Some('Z'),
            //numeric values to handle enter and backspace
            egui::Key::Enter => Some('1'),
            egui::Key::Backspace => Some('2'),
            egui::Key::Space => Some('3'),
            //for quick cheating
            egui::Key::Semicolon => Some('4'),
            _ => None,
        }
    }

    /// Function to handle keyboard inputs for letters
    fn handle_keyboard_input(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());

        for event in &input.events {
            if let egui::Event::Key { key, pressed, .. } = event {
                if *pressed {
                    if let Some(letter) = self.key_to_char(key) {
                        // check for a letter else it is an acction
                        if letter.is_ascii_alphabetic() && self.current_row < WORLD_LENGTH + 1 {
                            self.game_logic.current_guess[self.index] = letter.to_ascii_uppercase();
                            if self.index < WORLD_LENGTH - 1 {
                                self.index += 1;
                            }
                        } else {
                            let _ = match letter {
                                // enter -> submit guess
                                '1' => {
                                    match self.game_logic.submit_guess(self.dictionary.as_ref()) {
                                        (GameState::CorrectGuess, _) => {
                                            self.update_alphabet_state();
                                            self.current_row += 1;
                                            self.index = 0;
                                            self.game_state = Some(GameState::CorrectGuess);
                                        }
                                        (GameState::WrongGuess, word) => {
                                            println!("{}", GameState::WrongGuess);
                                            self.game_state = Some(GameState::WrongGuess);
                                            self.last_guessed_word = word;
                                        }
                                        (GameState::Lost, _) => {
                                            self.game_state = Some(GameState::Lost);
                                        }
                                        (GameState::Won, _) => {
                                            self.game_state = Some(GameState::Won);
                                        }
                                    }
                                }
                                // backspace -> delete letter
                                '2' => {
                                    // does the square already have a letter?
                                    if self.game_logic.current_guess[self.index] != ' ' {
                                        //remove it
                                        self.game_logic.current_guess[self.index] = ' '
                                    }
                                    //take a step back and delete
                                    else {
                                        if self.index == 0 {
                                        } else {
                                            self.index -= 1;
                                            self.game_logic.current_guess[self.index] = ' ';
                                        }
                                    }
                                }
                                // space -> new game
                                '3' => {
                                    self.new_game();
                                }
                                // for cheating
                                '4' => {
                                    println!("{}", self.word);
                                }
                                _ => {}
                            };
                        }
                    }
                }
            }
        }
    }

    /// Function to keep track of which letters have been used and their appearance in the word.
    fn update_alphabet_state(&mut self) {
        for (letters, states) in self
            .game_logic
            .guesses_letters
            .iter()
            .zip(self.game_logic.guesses.iter_mut())
        {
            for (letter, state) in letters.iter().zip(states.iter_mut()) {
                self.game_logic
                    .alphabet
                    .entry(*letter)
                    .and_modify(|e| *e = *state);
            }
        }
    }

    /// Function to consolidate all the updating on GUI
    fn update_visuals(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                let button_size = egui::vec2(80.0, 80.0);

                // Render the guesses grid
                for row in 0..6 {
                    ui.horizontal(|ui| {
                        for col in 0..5 {
                            let cell_color = match self.game_logic.guesses.get(row) {
                                Some(guess) => match guess[col] {
                                    LetterState::Correct => egui::Color32::GREEN,
                                    LetterState::Present => egui::Color32::YELLOW,
                                    LetterState::Wrong => egui::Color32::from_gray(50),
                                    LetterState::Unknown => egui::Color32::from_gray(50),
                                },
                                None => egui::Color32::from_gray(80),
                            };

                            let letter = if row == self.game_logic.current_row {
                                self.game_logic.current_guess[col].to_string()
                            } else {
                                match self.game_logic.guesses.get(row) {
                                    Some(_) => self.game_logic.guesses_letters[row][col].to_string(),
                                    None => " ".to_string(),
                                }
                            };

                            let text = if letter == " " { "_" } else { &letter };
                            match cell_color{
                                egui::Color32::GREEN => {
                                    let button = egui::Button::new(RichText::new(text).color(egui::Color32::BLACK))
                                    .min_size(button_size)
                                    .fill(cell_color);
                                    ui.add(button);
                                }
                                egui::Color32::YELLOW => {
                                    let button = egui::Button::new(RichText::new(text).color(egui::Color32::BLACK))
                                    .min_size(button_size)
                                    .fill(cell_color);
                                    ui.add(button);
                                }
                                _ => {
                                    let button = egui::Button::new(text)
                                    .min_size(button_size)
                                    .fill(cell_color);
                                    ui.add(button);
                                }
                            }

                        }
                    });
                    ui.add_space(5.0);
                }

                ui.add_space(20.0);

                // Render the alphabet grid
                ui.horizontal_wrapped(|ui| {
                    for letter in 'A'..='Z' {
                        let state = self.game_logic.alphabet.get(&letter).unwrap_or(&LetterState::Wrong);
                        let color = match state {
                            LetterState::Correct => egui::Color32::GREEN,
                            LetterState::Present => egui::Color32::YELLOW,
                            LetterState::Wrong => egui::Color32::from_gray(50),
                            LetterState::Unknown => egui::Color32::from_gray(80),
                        };

                        let button = egui::Button::new(letter.to_string())
                            .min_size(Vec2 { x: 40.0, y: 40.0 })
                            .fill(color);

                        ui.add(button);
                    }
                });

                ui.add_space(20.0);

                // Render the Submit and Restart buttons
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Submit Guess").clicked() {
                        match self.game_logic.submit_guess(self.dictionary.as_ref()){
                            (GameState::CorrectGuess, _) => {
                                self.update_alphabet_state();
                                self.current_row += 1;
                                self.index = 0;
                                self.game_state = Some(GameState::CorrectGuess);
                            }
                            (GameState::WrongGuess, word) => {
                                println!("{}", GameState::WrongGuess);
                                self.game_state = Some(GameState::WrongGuess);
                                self.last_guessed_word = word;
                            }
                            (GameState::Lost, _) => {
                                self.game_state = Some(GameState::Lost);
                                self.update_alphabet_state();
                                self.current_row += 1;
                            }
                            (GameState::Won, _) => {
                                self.game_state = Some(GameState::Won);
                                self.update_alphabet_state();
                                self.current_row += 1;
                            }
                        }
                    }

                    ui.add_space(20.0);

                    if ui.button("Restart").clicked() {
                        self.new_game();
                    }
                });

                ui.add_space(20.0);
                egui::ScrollArea::vertical().show(ui, |ui|{
                    match self.game_state {
                        Some(GameState::CorrectGuess) => {}
                        Some(GameState::WrongGuess) => {
                            let guessed_word: String = self.last_guessed_word.clone().unwrap();
                            ui.label(format!("{} is not long enough or not in the dicitonary", guessed_word));
                        }
                        Some(GameState::Lost) => {
                            ui.label(format!("Sorry you lost. The word was: {} \n Here's the defintion of the word if you are curious: \n {}", self.word, self.definition));
                        }
                        Some(GameState::Won) => {
                            ui.label(format!(
                                "Congratulations you won! Here's the definition of the word if you are curious: \n{}",
                                self.definition
                            ));
                        }
                        _ => {}
                    }
                })

            });
        });
    }
}

/// Eframe implementation for the app which is used for rendering
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_keyboard_input(ctx);
        self.update_visuals(ctx);
    }
}
