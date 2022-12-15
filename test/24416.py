from functools import reduce
import subprocess
from pathlib import Path

base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

for n in range(5, 41):
    proc = subprocess.Popen([base / "target/Debug/baekjoon.exe"], stdin=subprocess.PIPE)
    proc.stdin.write(str(n).encode("utf-8"))
    proc.stdin.close()
    proc.wait(timeout=2)
    proc.terminate()
