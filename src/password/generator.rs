use crate::password::{options::PasswordOptions, PasswordGenerator, consts::*};
use rand::random;
use PasswordOptions::{*};

#[derive(Debug)]
pub struct Generator {
    length: usize,
    options: Vec<PasswordOptions>,
    allowed_chars: Vec<char>,
}

impl PasswordGenerator for Generator {
    fn new(length: usize) -> Generator {
        Generator {
            length,
            options: Vec::new(),
            allowed_chars: Vec::new(),
        }
    }
    fn with_special(&mut self, required: Option<bool>) -> &mut Generator {
        self.options.push(Special { required: required.unwrap_or(false) });
        self
    }
    fn with_numbers(&mut self, required: Option<bool>) -> &mut Generator {
        self.options.push(Numbers { required: required.unwrap_or(false) });
        self
    }
    fn with_lowercase(&mut self, required: Option<bool>) -> &mut Generator {
        self.options.push(Lowercase { required: required.unwrap_or(false) });
        self
    }
    fn with_uppercase(&mut self, required: Option<bool>) -> &mut Generator {
        self.options.push(Uppercase { required: required.unwrap_or(false) });
        self
    }
    fn without_similar(&mut self) -> &mut Generator {
        self.options.push(NotSimilar);
        self
    }
    fn without_ambiguous(&mut self) -> &mut Generator {
        self.options.push(NotAmbiguous);
        self
    }
    fn without_sequential(&mut self) -> &mut Generator {
        self.options.push(NotSequential);
        self
    }
    fn generate(&mut self) -> String {
        self.allowed_chars = self.get_allowed_chars();
        let password = self.generate_password();
        password
    }
}

impl Generator {
    fn generate_password(&self) -> String {
        if self.length < MIN_PASSWORD_LENGTH {
            return format!("Password length must be at least {}", MIN_PASSWORD_LENGTH);
        }

        if self.allowed_chars.is_empty() {
            return format!("Password must contain at least one of the following: special, numbers, lowercase, uppercase");
        }

        let mut password = String::new();
        let mut valid = false;
        while !valid {
            for _ in 0..self.length {
                #[allow(unused_assignments)]
                let mut random_char: char = ' ';
                loop {
                    random_char = self.generate_random_char();
                    if !self.is_sequential(&random_char, password.clone()) {
                        break;
                    }
                }
                password.push(random_char);
            }
            valid = self.is_password_valid(&password);
        }
        password
    }

    fn generate_random_char(&self) -> char {
        self.allowed_chars[random::<usize>() % self.allowed_chars.len()]
    }

    fn is_password_valid(&self, password: &String) -> bool {
        let mut has_special = true;
        let mut has_number = true;
        let mut has_lowercase = true;
        let mut has_uppercase = true;
        for c in password.chars() {
            if self.options.contains(&Special {required: true}) && !SPECIAL_CHARS.contains(c) {
                has_special = false;
            } else if self.options.contains(&Numbers {required: true}) && !NUMERICAL_CHARS.contains(c) {
                has_number = false;
            } else if self.options.contains(&Lowercase {required: true}) && !LOWERCASE_CHARS.contains(c) {
                has_lowercase = false;
            } else if self.options.contains(&Uppercase {required: true}) && !UPPERCASE_CHARS.contains(c) {
                has_uppercase = false;
            }
        }
        self.options.contains(&Special {required: false}) || has_special &&
            self.options.contains(&Numbers {required: false}) || has_number &&
            self.options.contains(&Lowercase {required: false}) || has_lowercase &&
            self.options.contains(&Uppercase {required: false}) || has_uppercase
    }

    fn is_sequential(&self, random_char: &char, password: String) -> bool {
        if password.is_empty() {
            return false;
        }
        if self.options.contains(&NotSequential) {
            let char_index_result = self.allowed_chars.binary_search(&random_char);
            if char_index_result.is_err() {
                return false;
            }
            let char_index = char_index_result.unwrap();
            if char_index == 0 || char_index == self.allowed_chars.len() - 1 {
                return false;
            }
            let previous_char = self.allowed_chars[char_index - 1];
            let last_char = password.chars().last().unwrap();
            if previous_char == last_char {
                return true;
            }
            let next_char = self.allowed_chars[char_index + 1];
            if next_char == last_char {
                return true;
            }
        }
        false
    }

    fn get_allowed_chars(&self) -> Vec<char> {
        let mut allowed_chars = Vec::new();
        if self.options.contains(&Special { required: false } ) || self.options.contains(&Special { required: true }) {
            allowed_chars.extend(SPECIAL_CHARS.chars());
        }
        if self.options.contains(&Numbers { required: false }) || self.options.contains(&Numbers { required: true }) {
            allowed_chars.extend(NUMERICAL_CHARS.chars());
        }
        if self.options.contains(&Lowercase { required: false }) || self.options.contains(&Lowercase { required: true }) {
            allowed_chars.extend(LOWERCASE_CHARS.chars());
        }
        if self.options.contains(&Uppercase { required: false }) || self.options.contains(&Uppercase { required: true }){
            allowed_chars.extend(UPPERCASE_CHARS.chars());
        }
        if self.options.contains(&NotSimilar) {
            allowed_chars.retain(|c| !SIMILAR_CHARS.contains(*c));
        }
        if self.options.contains(&NotAmbiguous) {
            allowed_chars = allowed_chars
                .into_iter()
                .filter(|c| !AMBIGUOUS_SPECIAL_CHARS.contains(*c))
                .collect();
        }
        allowed_chars
    }
}
