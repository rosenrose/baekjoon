from functools import reduce
import subprocess
from pathlib import Path

base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

for d in range(3, 31):
    for k in range(10, 100_001):
        proc = subprocess.Popen(
            [base / "target/Debug/baekjoon.exe"], stdin=subprocess.PIPE, stdout=subprocess.PIPE
        )
        proc.stdin.write(f"{d} {k}".encode("utf-8"))
        proc.stdin.close()
        proc.wait(timeout=2)

        output = b""
        while bytes := proc.stdout.read():
            output += bytes

        output = output.decode("utf-8").splitlines()
        a, b = [int(i) for i in output[1:]]

        if 1 <= a and a <= b:
            print(f"{d=} {k=} {a=} {b=}, {output[0]}")

        proc.terminate()
