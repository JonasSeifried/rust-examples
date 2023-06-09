use std::collections::HashMap;
use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub enum GameState {
    InGame,
    GameWon,
    GameOver,
}

pub struct GameData {
    word_ans: String,
    map_guessed: HashMap<String, bool>,
    vec_wrongly_guessed: Vec<String>,
    guessed_count: i8,
    game_state: GameState,
    word_len: usize,
    wrongly_guessed_count: i8,
}

impl GameData {
    pub fn init() -> GameData {
        GameData {
            word_ans: String::new(),
            map_guessed: HashMap::new(),
            vec_wrongly_guessed: Vec::new(),
            guessed_count: 0,
            game_state: GameState::InGame,
            word_len: 0,
            wrongly_guessed_count: 0,
        }
    }

    pub fn handle_guess(&mut self, guess: &str) -> i8 {
        let count = self
            .word_ans
            .graphemes(true)
            .filter(|g| g.eq(&guess))
            .count();

        if count > 0 && !self.map_guessed.get(guess).unwrap_or(&false) {
            self.map_guessed.insert(guess.to_string(), true);
            self.guessed_count += count as i8;

            if self.guessed_count == self.word_len as i8 {
                self.game_state = GameState::GameWon;
            }
            return 0;
        }

        if !self.vec_wrongly_guessed.contains(&guess.to_string()) {
            self.vec_wrongly_guessed.push(guess.to_string());
            self.wrongly_guessed_count += 1;
            if self.wrongly_guessed_count == 6 {
                self.game_state = GameState::GameOver;
            }
            return 2;
        }
        1
    }

    pub fn scan_word(&mut self) -> bool {
        let bytes_read = io::stdin()
            .read_line(&mut self.word_ans)
            .expect("Failed to read Word");
        if bytes_read == 0 {
            return false;
        }
        self.word_ans = self.word_ans.trim().to_string();
        self.word_ans.graphemes(true).for_each(|g| {
            self.word_len += 1;
            if !self.map_guessed.contains_key(g) {
                self.map_guessed.insert(g.to_string(), false);
            }
        });
        true
    }

    pub fn word_to_string(&self) -> String {
        let mut output = "-> ".to_string();

        for g in self.word_ans.graphemes(true) {
            if *self.map_guessed.get(g).unwrap_or(&false) {
                output.push_str(g);
            } else {
                output.push_str(" _");
            }
        }
        if self.wrongly_guessed_count == 0 {
            return output;
        }
        output.push_str(" (");
        for grapheme in &self.vec_wrongly_guessed {
            output.push_str(grapheme);
            output.push_str(", ");
        }
        output.replace_range(output.len() - 2..output.len(), ")");
        output
    }

    pub fn hangman_to_string(&self) -> String {
        let mut output = String::new();
        if self.wrongly_guessed_count >= 1 {
            let mut g1 = "  |".to_string();
            if self.wrongly_guessed_count >= 2 {
                let mut g2 = "  |".to_string();
                if self.wrongly_guessed_count >= 3 {
                    let mut g3 = "  |".to_string();
                    if self.wrongly_guessed_count >= 4 {
                        if self.wrongly_guessed_count >= 5 {
                            if self.wrongly_guessed_count >= 6 {
                                g2 += "  0";
                                g1 += " /|\\";
                            }
                            g3 += "  |"
                        }
                        output.push_str("   ___\n");
                    }
                    output.push_str(&format!("{}\n", g3));
                }
                output.push_str(&format!("{}\n", g2));
            }
            output.push_str(&format!("{}\n", g1));
        }
        output.push_str("__|___\n");
        output.push_str("|     |\n");
        output
    }

    pub fn get_word_len(&self) -> usize {
        self.word_len
    }
    pub fn get_wrongly_guessed_count(&self) -> i8 {
        self.wrongly_guessed_count
    }
    pub fn get_state(&self) -> &GameState {
        &self.game_state
    }
}
