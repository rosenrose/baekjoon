import re

regex = re.compile(r"^[A-F]?A+F+C+[A-F]?$")

for _ in range(int(input())):
    print("Infected!" if regex.match(input().strip()) else "Good")
