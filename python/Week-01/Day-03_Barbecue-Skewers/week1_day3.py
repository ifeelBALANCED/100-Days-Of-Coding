def barbecue_skewers(skewers: list) -> list:
    """
    Count the number of vegetarian and non-vegetarian skewers.

    This function takes a list of skewers represented as strings, where each string contains 'x' for
    non-vegetarian items and 'o' for vegetarian items. It counts the number of vegetarian skewers
    (those that do not contain 'x') and non-vegetarian skewers.

    Parameters:
    skewers (list): A list of strings representing the skewers.

    Returns:
    list: A list containing two integers. The first integer is the number of vegetarian skewers,
          and the second integer is the number of non-vegetarian skewers.
    """
    vegetarian_skewers = 0
    for skewer in skewers:
        if "x" not in skewer:
            vegetarian_skewers += 1

    return [vegetarian_skewers, len(skewers) - vegetarian_skewers]
