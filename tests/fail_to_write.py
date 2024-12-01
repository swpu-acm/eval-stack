from pathlib import Path

Path.cwd().joinpath("test.txt").touch()
assert Path.cwd().joinpath("test.txt").exists()
Path.cwd().joinpath("test.txt").write_text("This is a test")
Path.cwd().joinpath("test.txt").write_bytes(b"This is a test")
assert Path.cwd().joinpath("test.txt").read_bytes() == b"This is a test"
assert Path.cwd().joinpath("test.txt").read_text() == "This is a test"
print(Path.cwd().joinpath("test.txt"))

a, b = map(int, input().split())
print(a + b)
print(a + b)
