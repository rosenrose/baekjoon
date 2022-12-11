from functools import reduce
import subprocess
from pathlib import Path

base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

for n in range(1, 1001):
    # print(reduce(lambda x, y: x * 2 + (1 if y % 2 == 0 else -1), range(2, n + 1), 0))

    proc = subprocess.Popen([base / "target/Debug/baekjoon.exe"], stdin=subprocess.PIPE)
    proc.stdin.write(str(n).encode("utf-8"))
    proc.stdin.close()
    proc.wait(timeout=2)
    proc.terminate()
