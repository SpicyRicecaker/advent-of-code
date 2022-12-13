# year, day
from sys import argv

if len(argv) != 3:
    print("Not enough args")
    quit()

base_path = f""
file_rs = f"{argv[1]}/src/bin/{argv[2]}.rs"
file_txt = f"{argv[1]}/{argv[2]}.txt"

# make sure not to overwrite existing files
try: 
    f = open(file_rs, 'x')
except Exception:
    pass

try: 
    f = open(file_txt, 'x')
except Exception:
    pass
