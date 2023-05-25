import subprocess
from pathlib import Path

base = Path(__file__) / "../../rust"
subprocess.run(["cargo", "build"], cwd=base)

for i in range(0, 100_001):
    for j in range(0, 100_001):
        input_str = f"{i} {j}"

        proc = subprocess.Popen([base / "target/Debug/baekjoon.exe"], stdin=subprocess.PIPE,stdout=subprocess.PIPE)
        proc.stdin.write(input_str.encode("utf-8"))
        proc.stdin.close()
        proc.wait(timeout=2)

        output = b""
        while bytes := proc.stdout.read():
            output += bytes

        output = output.decode("utf-8").strip()
        proc.terminate()

        print(f"{i} {j} {output}")
