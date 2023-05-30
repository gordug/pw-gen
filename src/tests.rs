use crate::password::consts::{*};
use crate::password::generator::Generator;
use crate::password::PasswordGenerator;

#[test]
fn it_works() {
    let password = Generator::new(10)
        .with_special(Some(true))
        .with_numbers(Some(true))
        .with_lowercase(Some(true))
        .with_uppercase(Some(true))
        .without_similar()
        .without_ambiguous()
        .without_sequential()
        .generate();
    println!("{}", password);
    assert_eq!(password.len(), 10);
}

#[test]
fn it_works_with_special() {
    let password = Generator::new(10).with_special(None).generate();
    assert_eq!(password.len(), 10);
    assert!(!contains_no_special_chars(&password));
    assert!(contains_no_numerical_chars(&password));
    assert!(contains_no_lowercase_chars(&password));
    assert!(contains_no_uppercase_chars(&password));
}

#[test]
fn it_works_with_numbers() {
    let password = Generator::new(10).with_numbers(None).generate();
    assert_eq!(password.len(), 10);
    assert!(contains_no_special_chars(&password));
    assert!(!contains_no_numerical_chars(&password));
    assert!(contains_no_lowercase_chars(&password));
    assert!(contains_no_uppercase_chars(&password));
}

#[test]
fn it_works_with_lowercase() {
    let password = Generator::new(10).with_lowercase(None).generate();
    assert_eq!(password.len(), 10);
    assert!(contains_no_special_chars(&password));
    assert!(contains_no_numerical_chars(&password));
    assert!(!contains_no_lowercase_chars(&password));
    assert!(contains_no_uppercase_chars(&password));
}

#[test]
fn it_works_with_uppercase() {
    let password = Generator::new(10).with_uppercase(None).generate();
    assert_eq!(password.len(), 10);
    assert!(contains_no_special_chars(&password));
    assert!(contains_no_numerical_chars(&password));
    assert!(contains_no_lowercase_chars(&password));
    assert!(!contains_no_uppercase_chars(&password));
}

#[test]
fn it_works_without_similar() {
    let password = Generator::new(10)
        .with_special(None)
        .with_numbers(None)
        .with_lowercase(None)
        .with_uppercase(None)
        .without_similar()
        .generate();
    println!("Password: {}", password);

    assert_eq!(password.len(), 10);
    assert!(contains_no_similar_chars(&password));
}

#[test]
fn it_works_without_ambiguous() {
    let password = Generator::new(10)
        .with_special(None)
        .with_numbers(None)
        .with_lowercase(None)
        .with_uppercase(None)
        .without_ambiguous()
        .generate();
    assert_eq!(password.len(), 10);
    assert!(contains_no_ambiguous_chars(&password));
}

#[test]
fn it_returns_warning_min_length() {
    let password = Generator::new(7).generate();
    assert_eq!(password, format!("Password length must be at least {}", MIN_PASSWORD_LENGTH));
}

#[test]
fn it_returns_warning_no_options() {
    let password = Generator::new(10).generate();
    assert_eq!(password, format!("Password must contain at least one of the following: special, numbers, lowercase, uppercase"));
}

fn contains_no_special_chars(password: &String) -> bool {
    for c in password.chars() {
        for x in SPECIAL_CHARS.chars() {
            if c == x {
                return false;
            }
        }
    }
    true
}

fn contains_no_numerical_chars(password: &String) -> bool {
    for c in password.chars() {
        for x in NUMERICAL_CHARS.chars() {
            if c == x {
                return false;
            }
        }
    }
    true
}

fn contains_no_lowercase_chars(password: &String) -> bool {
    for c in password.chars() {
        for x in LOWERCASE_CHARS.chars() {
            if c == x {
                return false;
            }
        }
    }
    true
}

fn contains_no_uppercase_chars(password: &String) -> bool {
    for c in password.chars() {
        for x in UPPERCASE_CHARS.chars() {
            if c == x {
                return false;
            }
        }
    }
    true
}

fn contains_no_similar_chars(password: &String) -> bool {
    for c in password.chars() {
        if SIMILAR_CHARS.contains(c) {
            return false;
        }
    }
    true
}

fn contains_no_ambiguous_chars(password: &String) -> bool {
    for c in password.chars() {
        if AMBIGUOUS_SPECIAL_CHARS.contains(c) {
            return false;
        }
    }
    true
}