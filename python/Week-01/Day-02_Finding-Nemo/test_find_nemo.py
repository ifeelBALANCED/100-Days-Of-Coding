import pytest
from week1_day2 import find_nemo


@pytest.mark.parametrize("input_sentence, expected_result", [
    ("I am finding Nemo !", "I found Nemo at 4!"),
    ("Nemo is me", "I found Nemo at 1!"),
    ("I Nemo am", "I found Nemo at 2!"),
    ("Hello, world", "I can't find Nemo :("),
    ("Nemo's friend is Dory", "I can't find Nemo :("),
    ("Nemo Nemo Nemo", "I found Nemo at 1!")
])
def test_find_nemo(input_sentence: str, expected_result: str) -> None:
    assert find_nemo(input_sentence) == expected_result
