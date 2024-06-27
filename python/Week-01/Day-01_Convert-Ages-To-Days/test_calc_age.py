import pytest
from week1_day1 import calc_age


@pytest.mark.parametrize("input_age_in_years, expected_days", [
    (0, 0),
    (1, 365),
    (2, 730),
    (65, 23725),
    (20, 7300),
])
def test_calc_age(input_age_in_years: int, expected_days: int) -> None:
    assert calc_age(input_age_in_years) == expected_days
