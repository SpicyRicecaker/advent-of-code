# year, day
from sys import argv
from pathlib import Path
from datetime import date

def create(path):
    try:
        _ = open(path, "x")
        print(f"successfully wrote to {path}")
    except Exception:
        print("did not write rust file since it already existed.")
        pass

if __name__ == "__main__":
    today = date.today()

    this_year = "twenty-three"
    print(f"year: (default: `{this_year}`)")
    year = input()
    year = year if year != "" else this_year

    print(f"day: (default `{today.day}`)")
    day = input()
    day = day if day != "" else today.day

    if len(argv) > 1 and argv[1] == '-e':
        this_part = 1
        print(f"part: (default `{this_part}`)")
        part = int(input())
        part = part if part != "" else this_part
        for path in [
            Path(f"{year}/{day}-{part}e.txt"),
        ]:
            create(path)
    else:
        for path in [
            Path(f"{year}/src/bin/{day}.rs"),
            Path(f"{year}/src/bin/{day}-2.rs"),
            Path(f"{year}/{day}.txt"),
        ]:
            create(path)
