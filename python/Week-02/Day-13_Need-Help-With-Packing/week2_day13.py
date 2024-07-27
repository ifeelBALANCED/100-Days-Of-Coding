def can_fit(weights: list, bags_number: int) -> bool:
    weights.sort(reverse=True)
    bags = [0] * bags_number
    for weight in weights:
        for i in range(len(bags)):
            if bags[i] + weight <= 10:
                bags[i] += weight
                break
        else:
            return False

    return True
