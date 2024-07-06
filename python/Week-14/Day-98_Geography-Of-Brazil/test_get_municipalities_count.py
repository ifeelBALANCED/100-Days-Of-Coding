import pytest
from unittest import mock
from week14_day98 import get_municipalities_count


@pytest.mark.parametrize("input_state, expected_result", [
    ("SP", 645),
    ("RJ", 92),
    ("MG", 853),
])
@mock.patch("httpx.get")
def test_get_municipalities_count(mock_get, input_state: str, expected_result: int) -> None:
    mock_response = mock_get.return_value
    # making mock return value to be an exact length as an actual response length
    mock_response.json.return_value = [{} for _ in range(expected_result)]

    assert get_municipalities_count(input_state) == expected_result
