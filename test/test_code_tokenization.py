import mlutil_rust


def test_tokenize_snakecase():
    snakecase_word = "a_function"
    assert mlutil_rust.tokenize_python_code(snakecase_word) == ["a", "function"]
