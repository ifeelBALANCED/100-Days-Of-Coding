from sympy import nextprime, isprime


def next_prime(number: int) -> int:
    if isprime(number):
        return number
    return nextprime(number)
