from itertools import combinations_with_replacement

m = int(input().split()[1])
arr = sorted(map(int, input().split()))

print(
    *[" ".join(map(str, i)) for i in sorted(set(combinations_with_replacement(arr, m)))], sep="\n"
)
