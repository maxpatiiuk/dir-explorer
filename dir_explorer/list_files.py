"""Add colors to the output of the `ls` command.

For usage examples, see `README.md`.
"""

import re
import subprocess
import sys

from color_definitions import (
    color_definitions,
    known_file_extension_endings,
    known_file_extensions,
    known_file_names,
)
from typing import Union, Callable


# CONFIG
# your "ls" command
ls = "CLICOLOR_FORCE=1 ls -GahlFT%"
# regex that separates file's metadata and file's name
regex = r"20\d\d "


try:
    result = subprocess.check_output(
        f"{ls} {' '.join(sys.argv[1:])}",
        shell=True,
    )
except subprocess.CalledProcessError:
    exit(1)


def resolve_color(
    unresolved_color: Union[str, int, Callable[[], Union[str, int]]]
) -> str:
    """Resolve color code to a complete escape sequence.

    param:
        unresolved_color:
            The colors that needs to be resolved.

    returns:
        Resolved string
        Color labels are resolved to color codes.
        Color codes are resolved to escape sequences.
        Custom strings that are not labels are returned as is.
        This allows for usage of arbitrary escape sequences
    """
    if callable(unresolved_color):
        return f"\x1B[{unresolved_color()}"
    if (
        type(unresolved_color) is str
        and unresolved_color in color_definitions
    ):
        return f"\x1B[38;5;{color_definitions[unresolved_color]}m"
    else:
        return unresolved_color


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
        file_extension_location = filename_part.rfind(".")
        if filename_part in known_file_names:
            filename_part = (
                f"{resolve_color(known_file_names[filename_part])}"
                f"{filename_part}"
                f"\x1B[0m"
            )
        elif file_extension_location != -1:
            file_extension = filename_part[
                file_extension_location + 1 :
            ]
            match = (
                known_file_extensions[file_extension]
                if file_extension in known_file_extensions
                else None
            )
            if not match:
                for (
                    file_extension_ending,
                    color,
                ) in known_file_extension_endings.items():
                    if file_extension.endswith(file_extension_ending):
                        match = color
            if match:
                filename_part = (
                    f"{filename_part[0:file_extension_location]}"
                    f"{resolve_color(match)}"
                    f"."
                    f"{file_extension}"
                    f"\x1B[0m"
                )

    print(f"\x1B[2m{meta_part}\x1b[0m{filename_part}")
