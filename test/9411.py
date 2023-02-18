import subprocess
import re
from pathlib import Path
from decimal import *
from random import randint

base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

getcontext().prec = 100


def randdecimal():
    num = "".join([str(randint(0, 9)) for _ in range(randint(2, 30))])
    point = randint(1, len(num) - 1)

    return Decimal(f"{'-' if randint(0, 1) == 1 else ''}{num[:point]}.{num[point:]}")


while True:
    a, b = randdecimal(), randdecimal()
    ans = re.sub(r"\.?0+$", "", str(a + b))

    proc = subprocess.Popen(
        [base / "target/Debug/baekjoon.exe"], stdin=subprocess.PIPE, stdout=subprocess.PIPE
    )
    proc.stdin.write(f"1\n{a}\n{b}\n0".encode("utf-8"))
    proc.stdin.close()
    proc.wait(timeout=2)

    output = b""
    while bytes := proc.stdout.read():
        output += bytes

    output = output.decode("utf-8").strip()

    if ans != output:
        print(f"{a} {b} {ans} {output}")

    proc.terminate()
