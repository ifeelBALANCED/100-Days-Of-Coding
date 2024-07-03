def check_if_prime(num):
    if num <= 1:
        return False
    if num <= 3:
        return True
    if num % 2 == 0 or num % 3 == 0:
        return False

    i = 5
    while i * i <= num:
        if num % i == 0 or num % (i + 2) == 0:
            return False
        i += 6
    return True


def next_prime(number: int) -> int:
    if check_if_prime(number):
        return number
    possible_prime = number
    while check_if_prime(possible_prime) is False:
        possible_prime += 1

    return possible_prime
