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
    (1000000, 1000003),
    (999983, 999983),
    (999984, 1000003),
    (10000000, 10000019),
    (100000000, 100000007),
    (1000000000, 1000000007),
])
def test_next_prime(input_number: int, expected_next_prime_number: int) -> None:
    assert next_prime(input_number) == expected_next_prime_number
