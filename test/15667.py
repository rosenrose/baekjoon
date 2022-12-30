import subprocess
from pathlib import Path

base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

for i in (Path(__file__) / "../15667_input.txt").read_text(encoding="utf-8").splitlines():
    proc = subprocess.Popen([base / "target/Debug/baekjoon.exe"], stdin=subprocess.PIPE)
    proc.stdin.write(i.encode("utf-8"))
    proc.stdin.close()
    proc.wait(timeout=2)

    proc.terminate()
