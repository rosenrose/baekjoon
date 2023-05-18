digits = 18
pow = 10**digits


def to_big(num):
    big = []

    while num > 0:
        big.append(num % pow)
        num //= pow

    return big


def to_int(big):
    return int(
        "".join(
            [
                f"{num}" if i == 0 else f"{num:0{digits}}"
                for i, num in enumerate(reversed(big))
            ]
        )
    )


n = int(input())
half = len(to_big(n)) // 2
lo, hi = to_int([pow - 1 for _ in range(half)]), to_int(
    [pow - 1 for _ in range(half + 1)]
)

while lo <= hi:
    mid = (lo + hi) // 2
    # print(f"{lo} {hi}")
    print(f"{to_big(mid)}")
    if mid * mid <= n:
        lo = mid + 1
    else:
        hi = mid - 1
