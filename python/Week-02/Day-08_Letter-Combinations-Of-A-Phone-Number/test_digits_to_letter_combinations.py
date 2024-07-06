import pytest
from week2_day8 import digits_to_letter_combinations


@pytest.mark.parametrize("input_digits, expected_result", [
    ("23", ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]),
    ("", []),
    ("2", ["a", "b", "c"]),
    ("9", ["w", "x", "y", "z"]),
    ("78", ["pt", "pu", "pv", "qt", "qu", "qv", "rt", "ru", "rv", "st", "su", "sv"]),
    ("234", ["adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi",
             "bdg", "bdh", "bdi", "beg", "beh", "bei", "bfg", "bfh", "bfi",
             "cdg", "cdh", "cdi", "ceg", "ceh", "cei", "cfg", "cfh", "cfi"]),
    ("2345", ["adgj", "adgk", "adgl", "adhj", "adhk", "adhl", "adij", "adik", "adil",
              "aegj", "aegk", "aegl", "aehj", "aehk", "aehl", "aeij", "aeik", "aeil",
              "afgj", "afgk", "afgl", "afhj", "afhk", "afhl", "afij", "afik", "afil",
              "bdgj", "bdgk", "bdgl", "bdhj", "bdhk", "bdhl", "bdij", "bdik", "bdil",
              "begj", "begk", "begl", "behj", "behk", "behl", "beij", "beik", "beil",
              "bfgj", "bfgk", "bfgl", "bfhj", "bfhk", "bfhl", "bfij", "bfik", "bfil",
              "cdgj", "cdgk", "cdgl", "cdhj", "cdhk", "cdhl", "cdij", "cdik", "cdil",
              "cegj", "cegk", "cegl", "cehj", "cehk", "cehl", "ceij", "ceik", "ceil",
              "cfgj", "cfgk", "cfgl", "cfhj", "cfhk", "cfhl", "cfij", "cfik", "cfil",])
])
def test_digits_to_letter_combinations(input_digits: str, expected_result: list) -> None:
    assert digits_to_letter_combinations(input_digits) == expected_result
