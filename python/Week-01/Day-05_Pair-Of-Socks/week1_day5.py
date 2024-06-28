def sock_pairs(socks: str) -> int:
    sock_pairs_count = 0
    for letter in set(socks):
        sock_pairs_count += socks.count(letter) // 2

    return sock_pairs_count
