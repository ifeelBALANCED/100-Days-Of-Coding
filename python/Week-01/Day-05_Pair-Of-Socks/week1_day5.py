def sock_pairs(socks: str) -> int:
    """
    Count the number of sock pairs.

    This function takes a string representing a collection of socks, where each character
    represents a sock of a particular color or type. It counts the number of pairs of socks
    in the collection.

    Parameters:
    socks (str): A string where each character represents a sock.

    Returns:
    int: The number of pairs of socks.
    """
    sock_pairs_count = 0
    for letter in set(socks):
        sock_pairs_count += socks.count(letter) // 2

    return sock_pairs_count
