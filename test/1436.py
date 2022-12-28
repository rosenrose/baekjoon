import subprocess
from pathlib import Path

base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

for i in range(1, 101):
    input_str = f"{i}"

    proc = subprocess.Popen([base / "target/Debug/baekjoon.exe"], stdin=subprocess.PIPE)
    proc.stdin.write(input_str.encode("utf-8"))
    proc.stdin.close()
    proc.wait(timeout=2)

    proc.terminate()
