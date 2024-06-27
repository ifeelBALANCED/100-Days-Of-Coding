def barbecue_skewers(skewers: list) -> list:
    vegetarian_skewers = 0
    non_vegetarian_skewers = 0
    for skewer in skewers:
        if "x" in skewer:
            non_vegetarian_skewers += 1
        else:
            vegetarian_skewers += 1

    return [vegetarian_skewers, non_vegetarian_skewers]
