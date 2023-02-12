import re

mass = {"C": 12.01, "H": 1.008, "O": 16.0, "N": 14.01}

for _ in range(int(input())):
    mole_sum = sum(
        [
            mass[atom] * (int(num) if num else 1)
            for atom, num in re.findall(r"([CHON])(\d*)", input())
        ]
    )

    print(f"{mole_sum:.3f}")
