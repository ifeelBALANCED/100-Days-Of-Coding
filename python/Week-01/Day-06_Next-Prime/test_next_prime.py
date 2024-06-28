import pytest
from week1_day6 import next_prime


@pytest.mark.parametrize("input_number, expected_next_prime_number", [
    (12, 13),
    (24, 29),
    (11, 11),
    (1, 2),
    (2, 2),
    (0, 2),
    (-1, 2),
    (17, 17),
    (18, 19),
])
def test_next_prime(input_number: int, expected_next_prime_number: int) -> None:
    assert next_prime(input_number) == expected_next_prime_number
