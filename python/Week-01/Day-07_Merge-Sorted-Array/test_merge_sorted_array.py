import pytest
from week1_day7 import merge_sorted_array


@pytest.mark.parametrize("nums1, nums2, expected_list", [
    ([1, 2, 3, 0, 0, 0], [2, 5, 6], [1, 2, 2, 3, 5, 6]),
    ([], [], []),
    ([0], [1], [1]),
    ([1, 2, 3, 0, 0, 0], [], [1, 2, 3, 0, 0, 0]),
    ([-1, 0, 0, 3, 3, 3, 0, 0, 0], [1, 2, 2], [-1, 0, 0, 1, 2, 2, 3, 3, 3]),
    ([4, 5, 6, 0, 0, 0], [1, 2, 3], [1, 2, 3, 4, 5, 6]),
    ([1], [], [1]),
    ([-10 ** 9, 0, 10 ** 9, 0, 0], [-10 ** 9, 10 ** 9], [-10 ** 9, -10 ** 9, 0, 10 ** 9, 10 ** 9]),
])
def test_merge_sorted_array(nums1: list, nums2: list, expected_list: list) -> None:
    assert merge_sorted_array(nums1, nums2) == expected_list
