def find_nemo(nemo_string: str) -> str:
    """
    Find the word 'Nemo' in a given string and return its position.

    This function searches for the word 'Nemo' in the provided string. If the word is found,
    it returns a message indicating the position (1-based index) of 'Nemo'. If the word is not found,
    it returns a message stating that 'Nemo' cannot be found.

    Parameters:
    nemo_string (str): The input string to search for the word 'Nemo'.

    Returns:
    str: A message indicating the position of 'Nemo' if found, or a message stating that 'Nemo' cannot be found.
    """
    split_nemo_string = nemo_string.split()
    for word in split_nemo_string:
        if word == "Nemo":
            nemo_index = split_nemo_string.index(word) + 1
            break
    else:
        return "I can't find Nemo :("

    return f"I found Nemo at {nemo_index}!"
