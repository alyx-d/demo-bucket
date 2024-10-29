import subprocess

process = subprocess.Popen(
    ["../golang/golang.exe"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
)

for line in process.stdout:
    print("output:", line, end='')

process.stdout.close()
process.stderr.close()
process.wait()
