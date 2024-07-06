def digits_to_letter_combinations(digits: str) -> list:
    if not digits:
        return []
    digits_repr = {
        "2": "abc",
        "3": "def",
        "4": "ghi",
        "5": "jkl",
        "6": "mno",
        "7": "pqrs",
        "8": "tuv",
        "9": "wxyz",
    }
    result_list = [""]

    for digit in digits:
        new_combinations = []
        for combination in result_list:
            for letter in digits_repr[digit]:
                new_combinations.append(combination + letter)
        result_list = new_combinations

    return result_list
