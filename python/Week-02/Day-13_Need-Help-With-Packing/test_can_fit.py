import pytest
from week2_day13 import can_fit


@pytest.mark.parametrize("input_weights, input_bags_number, expected_result", [
    ([2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2], 4, True),
    ([2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2], 4, False),
    ([10, 10, 10], 3, True),
    ([10, 10, 10], 2, False),
    ([1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, True),
    ([1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 2, True),
    ([5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 5, True),
    ([5, 5, 5, 5, 5, 5, 5, 5, 5, 5], 4, False),
    ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5, False),
    ([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4, False),
    ([1], 1, True),
    ([10], 1, True),
    ([10, 10, 10, 10], 4, True),
    ([10, 10, 10, 10], 3, False),
    ([9, 7, 4], 2, False),
])
def test_can_fit(input_weights: list, input_bags_number: int, expected_result: bool) -> None:
    assert can_fit(input_weights, input_bags_number) == expected_result
