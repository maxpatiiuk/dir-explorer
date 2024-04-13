# -*- coding: utf-8 -*-
"""Add colors to the output of the `ls` command.

For usage examples, see `README.md`.
"""

import re
import subprocess
import sys
import platform

from colorize_name import colorize_filename


# CONFIG
# Your "ls" command
ls = (
    "ls "
    "--color "
    "--group-directories-first "
    "-ahl "
    '--time-style=long-iso '
    "--sort=version"
)
if platform.system() == "Darwin":
    # Custom ls command when on macOS
    ls = f"g{ls}"


result = ""
try:
    result = subprocess.check_output(
        f"{ls} {' '.join(sys.argv[1:])}",
        shell=True,
    )
except subprocess.CalledProcessError:
    exit(1)

lines = result.rstrip().split(b"\n")
# last consistent space between lines:
# exclude the first line as it just shows the size
last_space = max([line.rfind(b" ") for line in lines[1:]])

for raw_line in lines:
    line = raw_line.decode("utf-8")

    meta_part = line[:last_space]
    filename_part = line[last_space:]

    if not filename_part:
        print(line)
        continue

    # links, executables and etc
    is_non_white = ord(filename_part[0]) == 27
    if is_non_white and not line.startswith("d"):
        meta_part = f"{meta_part[:-1]}\x1b[0m▸"

    # non-executable files
    elif line.startswith("-"):
        filename_part = colorize_filename(filename_part)

    print(f"\x1B[2m{meta_part}\x1b[0m{filename_part}")
