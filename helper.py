# year, day
from sys import argv
from pathlib import Path

print("year")
year = input()

print("day")
day = input()

file_rs = Path(f"{year}/src/bin/{day}.rs")
file_txt = Path(f"{year}/{day}.txt")

# make sure not to overwrite existing files
try: 
    f = open(file_rs, 'x')
    print(f"successfully wrote to {file_rs}")
except Exception:
    print("did not write rust file since it already existed.")
    pass

try: 
    f = open(file_txt, 'x')
    print(f"successfully wrote to {file_txt}")
except Exception:
    print("did not write text data file since it already existed.")
    pass
