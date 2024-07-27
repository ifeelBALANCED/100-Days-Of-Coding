import pytest
from week2_day14 import encrypt_karacas


@pytest.mark.parametrize("input_str, expected_output", [
    ("apple", "1lpp0aca"),
    ("banana", "0n0n0baca"),
    ("karaca", "0c0r0kaca"),
    ("burak", "k0r3baca"),
    ("alpaca", "0c0pl0aca"),
    ("", "aca"),
    ("a", "0aca"),
    ("b", "baca"),
    ("aeiou", "3o210aca"),
    ("bcdfg", "g2dcbaca"),
    ("racecar", "r0c1c0raca"),
    ("encyclopedia", "02d1polcycn1aca"),
    ("12345", "54321aca"),
    ("!@#$%", "%$#@!aca"),
    ("hello world", "dlrow oll1haca"),
])
def test_encrypt_karacas(input_str, expected_output):
    assert encrypt_karacas(input_str) == expected_output
