# year, day
from sys import argv
from pathlib import Path

print("year")
year = input()

print("day")
day = input()

file_rs = Path(f"{year}/src/bin/{day}.rs")
file_rs = Path(f"{year}/src/bin/{day}-2.rs")
file_txt = Path(f"{year}/{day}.txt")

for path in [file_rs, file_txt]:
    try:
        f = open(path, "x")
        print(f"successfully wrote to {path}")
    except Exception:
        print("did not write rust file since it already existed.")
        pass
