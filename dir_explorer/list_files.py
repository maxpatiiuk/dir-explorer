"""Add colors to the output of the `ls` command.

For usage examples, see `README.md`.
"""

import re
import subprocess
import sys

from colorize_name import colorize_filename


# CONFIG
# your "ls" command
ls = "CLICOLOR_FORCE=1 ls -GahlFT%"
# regex that separates file's metadata and file's name
regex = r"20\d\d "


result = ""
try:
    result = subprocess.check_output(
        f"{ls} {' '.join(sys.argv[1:])}",
        shell=True,
    )
except subprocess.CalledProcessError:
    exit(1)


for raw_line in result.rstrip().split(b"\n"):
    line = raw_line.decode("utf-8")
    match = re.search(regex, line)

    if not match:
        print(line)
        continue

    filename_begin = match.span()[1]
    meta_part = line[:filename_begin]
    filename_part = line[filename_begin:]

    # links, executables and etc
    if ord(filename_part[0]) == 27 and not line.startswith("d"):
        meta_part = f"{meta_part[:-1]}\x1b[0m▸"

    # non-executable files
    elif line.startswith("-"):
        filename_part = colorize_filename(filename_part)

    print(f"\x1B[2m{meta_part}\x1b[0m{filename_part}")
