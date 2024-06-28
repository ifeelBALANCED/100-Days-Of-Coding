def barbecue_skewers(skewers: list) -> list:
    vegetarian_skewers = 0
    for skewer in skewers:
        if "x" not in skewer:
            vegetarian_skewers += 1

    return [vegetarian_skewers, len(skewers) - vegetarian_skewers]
