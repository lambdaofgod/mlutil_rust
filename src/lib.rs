use pyo3::prelude::*;
use pyo3::types::*;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_clean_punctuation() {
        let s = "def abc(x)".into();
        let s_clean: String = "def abc x ".into();
        assert_eq!(remove_punctuation(s), s_clean)
    }

    #[test]
    fn test_clean_multiple_whitespaces() {
        let s = "a,b".into();
        let s_other = "a b".into();
        assert_eq!(
            remove_punctuation(s),
            remove_punctuation(s_other)
        )
    }

    #[test]
    fn test_tokenize() {
        let s = String::from("def abc(x)");
        let expected_tokens: Vec<&str> = vec!["def", "abc", "x"];
        let tokens = tokenize(s);
        for (token, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(token, expected)
        }
    }

    #[test]
    fn test_tokenize_snakecase() {
        let s = String::from("fun_name");
        let expected_tokens: Vec<&str> = vec!["fun", "name"];
        let tokens = tokenize(s);
        for (token, expected) in tokens.iter().zip(expected_tokens) {
            assert_eq!(token, expected)
        }
    }

    #[test]
    fn test_tokenize_signature() {
        let s = String::from("def fun_name(x: int):\npass");
        let expected_tokens: Vec<&str> = vec!["def", "fun", "name", "x", "int", "pass"];
        let tokens = tokenize(s);
        for (token, expected) in tokens.iter().zip(expected_tokens.iter()) {
            assert_eq!(&token, expected)
        }
    }
}

fn is_punctuation(ch: char) -> bool {
    const CHS: &'static [char] = &['(', ')', ',', '\"', '.', ';', ':', '\''];
    CHS.contains(&ch)
}

fn remove_punctuation(s: String) -> String {
    const WHITESPACE: char = ' ';
    s.chars()
        .map(|x| match is_punctuation(x) {
            true => WHITESPACE,
            false => x,
        })
        .collect()
}


fn tokenize_snakecase(word: String) -> Vec<String> {
    word.split("_").map(String::from).collect()
}

fn tokenize_word(word: String) -> Vec<String> {
    tokenize_snakecase(word)
}

fn tokenize(s: String) -> Vec<String> {
    let cleaned_str = remove_punctuation(s);
    let splits = cleaned_str.split_whitespace();
    let splits_iter = splits.flat_map(|s| tokenize_word(String::from(s)));
    splits_iter.collect()
}

#[pyfunction]
fn tokenize_python_code(python_code: &PyString) -> PyResult<Vec<String>> {
    let python_code_string: String = python_code.to_string(); // make sure the bytes are UTF-8
    Ok(tokenize(python_code_string))
}

#[pymodule]
fn mlutil_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(tokenize_python_code, m)?)?;
    Ok(())
}

fn main() {
}
