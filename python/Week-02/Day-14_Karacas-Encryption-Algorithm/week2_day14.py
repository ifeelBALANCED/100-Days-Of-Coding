def encrypt_karacas(input_word: str) -> str:
    vowels_to_num = {
        "a": "0",
        "e": "1",
        "i": "2",
        "f": "2",
        "u": "3",
    }
    return "".join([vowels_to_num.get(letter, letter) for letter in input_word[::-1]]) + "aca"
