def find_nemo(nemo_string: str) -> str:
    split_nemo_string = nemo_string.split()
    for word in split_nemo_string:
        if word == "Nemo":
            nemo_index = split_nemo_string.index(word) + 1
            break
    else:
        return "I can't find Nemo :("

    return f"I found Nemo at {nemo_index}!"
