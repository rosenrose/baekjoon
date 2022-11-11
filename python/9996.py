import re

N = int(input())
pattern = input().strip().split("*")
regex = re.compile(rf"^{pattern[0]}.*{pattern[-1]}$")

for _ in range(N):
    print("DA" if regex.match(input().strip()) else "NE")
