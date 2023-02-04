from functools import reduce

f = [
    None,
    "WH",
    ["WHWH", "WWHH"],
    ["WHWHWH", "WHWWHH", "WWHWHH", "WWWHHH", "WWHHWH"],
    [
        "WHWHWHWH",
        "WHWHWWHH",
        "WHWWHWHH",
        "WHWWWHHH",
        "WHWWHHWH",
        "WWHWHWHH",
        "WWHWWHHH",
        "WWWHWHHH",
        "WWWWHHHH",
        "WWWHHWHH",
        "WWHWHHWH",
        "WWWHHHWH",
        "WWHHWHWH",
        "WWHHWWHH",
    ],
]

result = []

result += [f"WH{i}" for i in f[4]]
result += [f"W{i}H" for i in f[4]]

result += [f"WH{i}{f[1]}" for i in f[3]]
result += [f"WH{f[1]}{i}" for i in f[3]]
result += [f"WH{i}{j}" for i in f[2] for j in f[2]]

result += [f"W{i}H{f[1]}" for i in f[3]]
result += [f"W{f[1]}H{i}" for i in f[3]]
result += [f"W{i}H{j}" for i in f[2] for j in f[2]]

result += [f"W{i}{f[1]}H" for i in f[3]]
result += [f"W{f[1]}{i}H" for i in f[3]]
result += [f"W{i}{j}H" for i in f[2] for j in f[2]]

result += [f"WH{i}{f[1]}{f[1]}" for i in f[2]]
result += [f"WH{f[1]}{i}{f[1]}" for i in f[2]]
result += [f"WH{f[1]}{f[1]}{i}" for i in f[2]]

result += [f"W{i}H{f[1]}{f[1]}" for i in f[2]]
result += [f"W{f[1]}H{i}{f[1]}" for i in f[2]]
result += [f"W{f[1]}H{f[1]}{i}" for i in f[2]]

result += [f"W{i}{f[1]}H{f[1]}" for i in f[2]]
result += [f"W{f[1]}{i}H{f[1]}" for i in f[2]]
result += [f"W{f[1]}{f[1]}H{i}" for i in f[2]]

result += [f"W{i}{f[1]}{f[1]}H" for i in f[2]]
result += [f"W{f[1]}{i}{f[1]}H" for i in f[2]]
result += [f"W{f[1]}{f[1]}{i}H" for i in f[2]]

result.append(f"WH{f[1]}{f[1]}{f[1]}{f[1]}")
result.append(f"W{f[1]}H{f[1]}{f[1]}{f[1]}")
result.append(f"W{f[1]}{f[1]}H{f[1]}{f[1]}")
result.append(f"W{f[1]}{f[1]}{f[1]}H{f[1]}")
result.append(f"W{f[1]}{f[1]}{f[1]}{f[1]}H")

print(*result, sep="\n")
print(len(result), len(set(result)))

result2 = reduce(lambda x, y: x if y in x else [*x, y], result, [])
print(*result2, sep="\n")

"""
WHWHWHWHWH
WHWHWHWWHH
WHWHWWHWHH
WHWHWWWHHH
WHWHWWHHWH
WHWWHWHWHH
WHWWHWWHHH
WHWWWHWHHH
WHWWWWHHHH
WHWWWHHWHH
WHWWHWHHWH
WHWWWHHHWH
WHWWHHWHWH
WHWWHHWWHH

WWHWHWHWHH
WWHWHWWHHH
WWHWWHWHHH
WWHWWWHHHH
WWHWWHHWHH
WWWHWHWHHH
WWWHWWHHHH
WWWWHWHHHH
WWWWWHHHHH
WWWWHHWHHH
WWWHWHHWHH
WWWWHHHWHH
WWWHHWHWHH
WWWHHWWHHH

WHWHWHWHWH
WHWHWWHHWH
WHWWHWHHWH
WHWWWHHHWH
WHWWHHWHWH

WHWHWHWHWH
WHWHWHWWHH
WHWHWWHWHH
WHWHWWWHHH
WHWHWWHHWH

WHWHWHWHWH
WHWHWHWWHH
WHWWHHWHWH
WHWWHHWWHH

WWHWHWHHWH
WWHWWHHHWH
WWWHWHHHWH
WWWWHHHHWH
WWWHHWHHWH

WWHHWHWHWH
WWHHWHWWHH
WWHHWWHWHH
WWHHWWWHHH
WWHHWWHHWH

WWHWHHWHWH
WWHWHHWWHH
WWWHHHWHWH
WWWHHHWWHH

WWHWHWHWHH
WWHWWHHWHH
WWWHWHHWHH
WWWWHHHWHH
WWWHHWHWHH

WWHWHWHWHH
WWHWHWWHHH
WWHWWHWHHH
WWHWWWHHHH
WWHWWHHWHH

WWHWHWHWHH
WWHWHWWHHH
WWWHHWHWHH
WWWHHWWHHH

WHWHWHWHWH
WHWWHHWHWH
WHWHWHWHWH
WHWHWWHHWH
WHWHWHWHWH
WHWHWHWWHH

WWHWHHWHWH
WWWHHHWHWH
WWHHWHWHWH
WWHHWWHHWH
WWHHWHWHWH
WWHHWHWWHH

WWHWHWHHWH
WWWHHWHHWH
WWHWHWHHWH
WWHWWHHHWH
WWHWHHWHWH
WWHWHHWWHH

WWHWHWHWHH
WWWHHWHWHH
WWHWHWHWHH
WWHWWHHWHH
WWHWHWHWHH
WWHWHWWHHH

WHWHWHWHWH
WWHHWHWHWH
WWHWHHWHWH
WWHWHWHHWH
WWHWHWHWHH
----------
WHWHWHWHWH
WHWHWHWWHH
WHWHWWHWHH
WHWHWWWHHH
WHWHWWHHWH
WHWWHWHWHH
WHWWHWWHHH
WHWWWHWHHH
WHWWWWHHHH
WHWWWHHWHH
WHWWHWHHWH
WHWWWHHHWH
WHWWHHWHWH
WHWWHHWWHH
WWHWHWHWHH
WWHWHWWHHH
WWHWWHWHHH
WWHWWWHHHH
WWHWWHHWHH
WWWHWHWHHH
WWWHWWHHHH
WWWWHWHHHH
WWWWWHHHHH
WWWWHHWHHH
WWWHWHHWHH
WWWWHHHWHH
WWWHHWHWHH
WWWHHWWHHH
WWHWHWHHWH
WWHWWHHHWH
WWWHWHHHWH
WWWWHHHHWH
WWWHHWHHWH
WWHHWHWHWH
WWHHWHWWHH
WWHHWWHWHH
WWHHWWWHHH
WWHHWWHHWH
WWHWHHWHWH
WWHWHHWWHH
WWWHHHWHWH
WWWHHHWWHH
"""
