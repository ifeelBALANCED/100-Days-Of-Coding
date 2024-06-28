import pytest
from week1_day5 import sock_pairs


@pytest.mark.parametrize("input_socks, expected_sock_pairs_count", [
    ("AA", 1),
    ("ABABC", 2),
    ("CABBACCC", 4),
    ("", 0),
    ("ABC", 0),
    ("AABBA", 2),
    ("AAAA", 2),
    ("AABBB", 2),
])
def test_sock_pairs(input_socks: str, expected_sock_pairs_count: int) -> None:
    assert sock_pairs(input_socks) == expected_sock_pairs_count
