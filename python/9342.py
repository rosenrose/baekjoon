import re

T = int(input())
regex = re.compile(r"^[A-F]?A+F+C+[A-F]?$")

for _ in range(T):
    print("Infected!" if regex.match(input().strip()) else "Good")
