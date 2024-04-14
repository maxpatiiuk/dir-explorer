"""Colorize a filename."""

from typing import Callable, Union
from color_definitions import (
    color_definitions,
    known_file_extension_endings,
    known_file_extensions,
    known_file_names,
)


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


def colorize_filename(filename: str) -> str:
    """Colorize a filename based on it's extension.

    param:
        filename: File name with extension

    returns:
        File name string with color escape sequences
    """
    if filename in known_file_names:
        return (
            f"{resolve_color(known_file_names[filename])}"
            f"{filename}"
            f"\x1B[0m"
        )

    file_extension_location = filename.rfind(".")
    if file_extension_location != -1:
        file_extension = filename[file_extension_location + 1 :]
        lower_file_extension = file_extension.lower()

        match = (
            known_file_extensions[lower_file_extension]
            if lower_file_extension in known_file_extensions
            else None
        )

        if not match:
            for (
                file_extension_ending,
                color,
            ) in known_file_extension_endings.items():
                if lower_file_extension.endswith(file_extension_ending):
                    match = color

        if match:
            return (
                f"{filename[0:file_extension_location]}"
                f"{resolve_color(match)}"
                f"."
                f"{file_extension}"
                f"\x1B[0m"
            )

    return filename
