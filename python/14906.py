import re

slump = r"((D|E)F+)+G"
slimp = rf"(?P<ab>(AB)*)A(H|{slump}C)(?P<c>C*)"
slurpy = re.compile(rf"^{slimp}{slump}$")

print("SLURPYS OUTPUT")

for _ in range(int(input())):
    if not (match := slurpy.match(input())):
        print("NO")
        continue

    print("YES" if match["ab"].count("AB") == match["c"].count("C") else "NO")

print("END OF OUTPUT")
