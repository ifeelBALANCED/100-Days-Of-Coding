import pytest
from week1_day4 import johny_progress_days


@pytest.mark.parametrize("input_miles, expected_progress_days", [
    ([3, 4, 1, 2], 2),
    ([10, 11, 12, 9, 10], 3),
    ([6, 5, 4, 3, 2, 9], 1),
    ([9, 9], 0),
    ([1, 2, 3, 4, 5], 4),
    ([5, 4, 3, 2, 1], 0),
    ([1, 1, 1, 1, 1], 0),
    ([], 0),
    ([1], 0),
    ([2, 2, 3, 3, 4, 4], 2)
])
def test_johny_progress_days(input_miles: list, expected_progress_days: int) -> None:
    assert johny_progress_days(input_miles) == expected_progress_days
