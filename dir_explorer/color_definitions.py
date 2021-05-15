"""Define assiciations between file extensions and colors.

Instructions on editing this file are provided in `README.md`.
"""

from typing import Callable


def custom_color(x: int) -> Callable[[], str]:
    """Return an escape string for a color by it's number.

    param:
        x:
            The color code (between 0 and 255).

    returns:
        Formatted escape sequence for a color
    """
    return lambda: "38;5;" + str(x) + "m"


color_definitions = {
    "gray": "8",
    "purple": "13",
    "red": "1",
    "green": "2",
    "yellow": "11",
    "blue": "18",
    "dirty-red": "88",
    "dirty-yellow": "100",
    "dirty-green": "119",
    "pink": "125",
}

known_file_extensions = {
    # ignored / auto-generated
    "DS_Store": "gray",
    "old": "gray",
    "swp": "gray",
    "tmp": "gray",
    "temp": "gray",
    "bak": "gray",
    "bkp": "gray",
    "log": "gray",
    # important files
    "pub": "red",
    "private": "red",
    "key": "red",
    # config
    "cfg": "purple",
    "conf": "purple",
    "ini": "purple",
    "properties": "purple",
    "config": "purple",
    # c-like
    "asm": "yellow",
    "c": "yellow",
    "class": "yellow",
    "cpp": "yellow",
    "cs": "yellow",
    "h": "yellow",
    "hpp": "yellow",
    "php": "yellow",
    "jar": "yellow",
    # windows
    "bat": "yellow",
    "exe": "yellow",
    "bin": "yellow",
    "wsf": "yellow",
    "msi": "yellow",
    # source text
    "csv": "blue",
    "json": "blue",
    "txt": "blue",
    "xml": "blue",
    "yaml": "blue",
    "yml": "blue",
    "dat": "blue",
    # very common files
    "css": custom_color(123),  # blue
    "htm": "green",
    "html": "green",
    "js": custom_color(208),  # orange
    "jsx": custom_color(202),  # dark orange
    "md": custom_color(200),  # purple
    "py": custom_color(119),  # pale green
    "ts": custom_color(153),  # blue
    "tsx": custom_color(81),  # dirty blue
    "sql": custom_color(94),  # dirty yellow
    # images
    "jpg": "pink",
    "jpeg": "pink",
    "svg": "pink",
    "png": "pink",
    "gif": "pink",
    "bmp": "pink",
    "webp": "pink",
    "tif": "pink",
    "tiff": "pink",
    "psd": "pink",
    "ai": "pink",
    "ico": "pink",
    "heic": "pink",
    # executables
    "dmg": "yellow",
    "iso": "yellow",
    # media
    "aif": "dirty-green",
    "mp3": "dirty-green",
    "wav": "dirty-green",
    "mp4": "dirty-green",
    "avi": "dirty-green",
    "mov": "dirty-green",
    "otf": "dirty-green",
    "ttf": "dirty-green",
    "mkv": "dirty-green",
    "mpg": "dirty-green",
    "mpeg": "dirty-green",
    "wmv": "dirty-green",
    "m4a": "dirty-green",
    # archives
    "pkg": "dirty-yellow",
    "apk": "dirty-yellow",
    "deb": "dirty-yellow",
    "rar": "dirty-yellow",
    "zip": "dirty-yellow",
    "7z": "dirty-yellow",
    "tar": "dirty-yellow",
    "gz": "dirty-yellow",
    "xz": "dirty-yellow",
    "hz": "dirty-yellow",
    # documents
    "doc": "dirty-red",
    "docx": "dirty-red",
    "pdf": "dirty-red",
    "ppt": "dirty-red",
    "pptx": "dirty-red",
    "xsl": "dirty-red",
    "xslx": "dirty-red",
    "keynote": "dirty-red",
}

known_file_extension_endings = {
    "ignore": "gray",
    "_history": "gray",
    "hst": "gray",
    "info": "yellow",
    "config": "yellow",
    "rc": "yellow",
    "sh": "yellow",
}

known_file_names = {
    # ignored
    "LICENSE": "gray",
    "__init__.py": "gray",
    "package-lock.json": "gray",
    # important files
    "README.md": "red",
    "Makefile": "red",
    "Dockerfile": "red",
    "docker-compose.yml": "red",
    ".pre-commit-config.yaml": "red",
    ".pre-commit-hooks.yaml": "red",
    "webpack.config.js": "red",
    "package.json": "red",
    "tsconfig.json": "red",
    "tsconfig.eslint.json": "red",
    "tsconfig.tests.json": "red",
    "requirements.txt": "red",
    "requirements-testing.txt": "red",
}
