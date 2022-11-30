import subprocess
from pathlib import Path

base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

for i in range(3, 100_001):
    proc = subprocess.Popen([base / "target/Debug/baekjoon.exe"], stdin=subprocess.PIPE)
    input_str = f"1\n{i}"
    proc.stdin.write(input_str.encode("utf-8"))
    proc.stdin.close()

    proc.wait()
    proc.terminate()
