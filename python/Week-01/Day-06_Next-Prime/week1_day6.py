from sympy import nextprime, isprime


def next_prime(number: int) -> int:
    """
    Find the next prime number greater than or equal to a given number.

    This function checks if the given number is prime. If it is prime, it returns the number itself.
    If it is not prime, it finds and returns the smallest prime number greater than the given number.

    Parameters:
    number (int): The number for which the next prime number is to be found.

    Returns:
    int: The next prime number greater than or equal to the given number.
    """
    if isprime(number):
        return number
    return nextprime(number)
