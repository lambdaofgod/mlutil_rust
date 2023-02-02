use std::error::Error;

use lazy_static::lazy_static;
use regex;
use regex::{Captures, Regex};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_clean_punctuation() {
        let s = "def abc(x)".into();
        let s_clean: String = "def abc ( x )".into();
        assert_eq!(handle_punctuation(s).trim(), s_clean.trim())
    }

    #[test]
    fn test_clean_multiple_whitespaces() {
        let s = "a,b".into();
        let s_other: String = "a , b".into();
        assert_eq!(handle_punctuation(s), s_other)
    }

    #[test]
    fn test_tokenize_python_code() {
        let s = String::from("def abc(x)");
        let expected_tokens: Vec<&str> = vec!["def", "abc", "(", "x", ")"];
        let tokens = tokenize_python_code(s);
        for (token, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(token, expected)
        }
    }

    #[test]
    fn test_tokenize_snakecase() {
        let s = String::from("fun_name");
        let expected_tokens: Vec<&str> = vec!["fun", "name"];
        let tokens = tokenize_python_code(s);
        for (token, expected) in tokens.iter().zip(expected_tokens) {
            assert_eq!(token, expected)
        }
    }

    #[test]
    fn test_tokenize_signature() {
        let s = String::from("def fun_name(x: int):\npass");
        let expected_tokens: Vec<&str> =
            vec!["def", "fun", "name", "(", "x", ":", "int", ")", ":", "pass"];
        let tokens = tokenize_python_code(s);
        for (token, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(&token, expected)
        }
    }

    #[test]
    fn test_tokenize_string_constants() {
        let s = String::from("'x'");
        let expected_tokens: Vec<&str> = vec!["'x'"];
        let tokens = tokenize_python_code(s);
        for (token, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(&token, expected)
        }
    }

    #[test]
    fn test_tokenize_camelcase() {
        let s = String::from("AbstractFactoryProps");
        let expected_tokens: Vec<&str> = vec!["abstract", "factory", "props"];
        let tokens = tokenize_python_code(s);
        for (token, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(&token, expected)
        }
    }

    #[test]
    fn test_tokenize_special_methods() {
        let s = String::from("__init__(self)");
        let expected_tokens: Vec<&str> = vec!["init", "(", "self", ")"];
        let tokens = tokenize_python_code(s);
        for (token, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(&token, expected)
        }
    }

    #[test]
    fn test_tokenize_methods_with_strings() {
        let s = String::from("f(x = \"12\")");
        let expected_tokens: Vec<&str> = vec!["f", "(", "x", "=", "\"", "NUMBER", "\"", ")"];
        let tokens = tokenize_python_code(s);
        for (token, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(&token, expected)
        }
    }
}

fn is_punctuation(ch: char) -> bool {
    CHS.contains(&ch)
}

lazy_static! {
    static ref CHS: &'static [char] =
        &['(', ')', ',', '.', ';', ':', '=', '[', ']', '{', '}', '-', '+', '_'];
    static ref CAMELCASE_REGEX: Regex = regex::Regex::new(r"([a-z])([A-Z])").unwrap();
    static ref NUMBER_REGEX: Regex = regex::Regex::new(r"(\d+)").unwrap();
}

fn clean_numbers(s: String) -> String {
    let result = NUMBER_REGEX.replace_all(&s, |caps: &Captures| " NUMBER ");
    result.to_string()
}

fn clean_camelcase(s: String) -> String {
    let result =
        CAMELCASE_REGEX.replace_all(&s, |caps: &Captures| format!("{} {}", &caps[1], &caps[2]));
    result.to_string()
}

fn handle_punctuation(s: String) -> String {
    const WHITESPACE: char = ' ';
    let mut new_s = String::new();
    for ch in s.chars() {
        if is_punctuation(ch) {
            new_s.push(' ');
            new_s.push(ch);
            new_s.push(' ')
        } else {
            new_s.push(ch)
        }
    }
    new_s
}

fn tokenize_snakecase(word: String) -> Vec<String> {
    word.split("_")
        .map(|w| String::from(w.trim()))
        .filter(|w| w != "")
        .collect()
}

fn tokenize_word(word: String) -> Vec<String> {
    tokenize_snakecase(word)
}

pub fn tokenize_python_code(s: String) -> Vec<String> {
    let cleaned_camelcase_str = clean_camelcase(s);
    let cleaned_str = clean_numbers(handle_punctuation(cleaned_camelcase_str).to_lowercase());
    let splits = cleaned_str.split_whitespace();
    let mut acc: Vec<String> = Vec::new();
    for split in splits {
        for w in tokenize_word(String::from(split.trim())) {
            acc.push(w);
        }
    }
    acc
}
