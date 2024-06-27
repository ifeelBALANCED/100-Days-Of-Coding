import pytest
from week1_day3 import barbecue_skewers


@pytest.mark.parametrize("input_skewers, expected_output", [
    (["--xo--x--ox--", "--xx--x--xx--", "--oo--o--oo--", "--xx--x--ox--", "--xx--x--ox--"], [1, 4]),
    (["--oooo-ooo--", "--xx--x--xx--", "--o---o--oo--", "--xx--x--ox--", "--xx--x--ox--"], [2, 3]),
    (["--oooo-ooo--", "--xxxxxxxx--", "--o---", "-o-----o---x--", "--o---o-----"], [3, 2]),
    (["--o--o--o--", "--x--x--x--", "--o--o--o--", "--x--x--x--", "--o--o--o--"], [3, 2]),
    (["--o--o--o--", "--x--x--x--", "--o--o--o--", "--x--x--x--", "--x--x--x--"], [2, 3]),
    (["--o--o--o--", "--x--x--x--", "--o--o--o--", "--o--o--o--", "--o--o--o--"], [4, 1])
])
def test_barbecue_skewers(input_skewers: list, expected_output: list) -> None:
    assert barbecue_skewers(input_skewers) == expected_output
