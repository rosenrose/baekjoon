from itertools import combinations_with_replacement

m = int(input().split()[1])

print(
    *[
        " ".join(map(str, i))
        for i in combinations_with_replacement(sorted(map(int, input().split())), m)
    ],
    sep="\n"
)
