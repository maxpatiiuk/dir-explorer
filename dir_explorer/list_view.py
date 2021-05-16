"""Show files in a directory in a list view."""

import os
import sys
import curses
import platform
import subprocess
import json
from enum import Enum
from typing import List, Tuple, TypedDict, Dict
from binaryornot.check import is_binary

DEBUG = False

# DEFINITIONS
home_directory = os.path.expanduser("~")


class Action(Enum):
    """ENUM of actions a user can take."""

    UP = "UP"
    DOWN = "DOWN"
    LEFT = "LEFT"
    RIGHT = "RIGHT"
    REDRAW = "REDRAW"
    QUIT = "QUIT"
    OPEN = "OPEN"
    OPEN_DEFAULT = "OPEN_DEFAULT"
    EDIT = "EDIT"
    PREVIEW = "PREVIEW"
    DEFAULT = "DEFAULT"


map_keys: Dict[int, Action] = {
    curses.KEY_UP: Action.UP,
    curses.KEY_DOWN: Action.DOWN,
    curses.KEY_LEFT: Action.LEFT,
    curses.KEY_RIGHT: Action.RIGHT,
    curses.KEY_RESIZE: Action.REDRAW,
    -1: Action.REDRAW,  # error
    ord(" "): Action.OPEN_DEFAULT,
    curses.KEY_ENTER: Action.OPEN_DEFAULT,
    ord("o"): Action.OPEN,
    ord("O"): Action.OPEN,
    ord("p"): Action.PREVIEW,
    ord("P"): Action.PREVIEW,
    ord("e"): Action.OPEN,
    ord("E"): Action.OPEN,
    ord("w"): Action.UP,
    ord("W"): Action.UP,
    ord("s"): Action.DOWN,
    ord("S"): Action.DOWN,
    ord("a"): Action.LEFT,
    ord("A"): Action.LEFT,
    ord("d"): Action.RIGHT,
    ord("D"): Action.RIGHT,
    ord("q"): Action.QUIT,
    ord("Q"): Action.QUIT,
    ord("k"): Action.UP,
    ord("K"): Action.UP,
    ord("j"): Action.DOWN,
    ord("J"): Action.DOWN,
    ord("h"): Action.LEFT,
    ord("H"): Action.LEFT,
    ord("l"): Action.RIGHT,
    ord("L"): Action.RIGHT,
}


class ItemProps(TypedDict):
    """File properties."""

    is_dir: bool


def get_items(path: str) -> List[Tuple[str, ItemProps]]:
    """Get a list of files and folders.

    param:
        path: The path to search in

    returns:
        List of files and folders
    """
    items = [
        (
            item,
            ItemProps(is_dir=os.path.isdir(os.path.join(path, item))),
        )
        for item in os.listdir(path)
    ]

    items.sort(key=lambda item: str(not item[1]["is_dir"]) + item[0])

    return items


def trim_string(
    string: str,
    limit: int,
    max_length: int,
    trim_back: bool,
) -> str:
    """Trip a string to fit in viewport.

    param:
        string: The string to trim
        limit: The available screen estate (horizontal)
        max_length: The length of the longest file name in a directory
        trim_back: Whether to trim the end or the beginning of the string

    return:
        Trimmed string
    """
    if len(string) > limit:
        if trim_back:
            return string[0 : limit - 1] + "…"
        else:
            return "…" + string[-limit + 1 :]
    else:
        return string + " " * (min(max_length, limit) - len(string))


def display_path(
    stdscr,
    items: List[Tuple[str, ItemProps]],
    path: str,
    selected: int,
    height_limit: int,
    width_limit: int,
) -> None:
    """Render a list of files and directories.

    param:
        stdscr: Window object
        items: Directory's contents
        path: Directory path
        selected: Index of selected item
        height_limit: Height of the screen
        width_limit: Width of the screen
    """
    relative_path = (
        "~{}".format(path[len(home_directory) :])
        if path.startswith(home_directory)
        else path
    )

    max_length = max(
        [len(item) for item, item_props in [*items, (path, {})]]
    )

    stdscr.addstr(
        0,
        0,
        trim_string(relative_path, width_limit, max_length, False),
        curses.A_REVERSE,
    )

    lines = height_limit - 1
    middle = lines // 2

    # If there are more items then free space on the screen, show a subset of
    # the items
    if selected <= middle or len(items) <= lines:
        index_offset = 0
        items = items[0:lines]
    elif len(items) - selected <= middle:
        index_offset = len(items) - lines
        items = items[-lines:]
    else:
        index_offset = selected - middle
        items = items[selected - middle : selected + middle]

    for index, (item, item_props) in enumerate(items):
        trimmed_string = trim_string(
            item, width_limit, max_length, True
        )
        # TODO: add support for color_definitions.py
        stdscr.addstr(
            index + 1,
            0,
            trimmed_string,
            curses.color_pair(
                1
                if index_offset + index == selected
                else 2
                if item_props["is_dir"]
                else 0
            ),
        )


def terminate_curses(stdscr) -> None:
    """Reset terminal's state back to normal.

    param:
        stdscr: Window object
    """
    curses.nocbreak()
    curses.echo()
    curses.endwin()
    stdscr.keypad(False)


def config(stdscr) -> None:
    """Configure the terminal for curses.

    param:
        stdscr: Window object
    """
    curses.noecho()
    curses.cbreak()
    curses.start_color()
    stdscr.keypad(True)
    curses.curs_set(False)  # disable the cursor

    # initialize colors [(fg, bg)]
    curses.init_pair(
        1, curses.COLOR_BLACK, curses.COLOR_CYAN
    )  # current item
    curses.init_pair(2, curses.COLOR_CYAN, curses.COLOR_BLACK)  # folder


def open_file(file) -> None:
    """Open a file using default program.

    param:
        file: Path to a file
    """
    if platform.system() == "Darwin":  # macOS
        subprocess.call(("open", file))
    elif platform.system() == "Windows":  # Windows
        os.startfile(file)
    else:  # linux variants
        subprocess.call(("xdg-open", file))


def main(
    stdscr,
    path: str,
    index_stack: List[Tuple[str, int]] = None,
    stack_position: int = 0,
) -> None:
    """Begin the main loop of the program.

    Listens for keys and manages the stack of directories.

    param:
        stdscr: Window object
        path: Initial path
        index_stack:
            The visited directories and the index of the selected item in each
        stack_position:
            Current position in the index_stack
    """
    height_limit, width_limit = stdscr.getmaxyx()
    stdscr.clear()

    if not index_stack:
        index_stack = []
    index = 0

    # Get list of files and directories
    items = get_items(path)

    # Open parent directory
    if stack_position == -1:

        # Find the index of the initial directory inside of the parent
        child_index = 0
        for index, item in enumerate(items):
            if os.path.join(path, item[0]) == index_stack[0][0]:
                child_index = index
                break

        index_stack.insert(0, [path, child_index])
        stack_position = 0

    # Open child directory that has been visited before
    elif stack_position < len(index_stack):
        if index_stack[stack_position][0] == path:
            index = index_stack[stack_position][1]
        else:
            index_stack = index_stack[0 : stack_position + 1]

    # Open new child directory
    else:
        index_stack.append([path, index])

    while True:
        index = index % (len(items) if items else 1)

        index_stack[stack_position] = [path, index]

        # Update the screen
        display_path(
            stdscr, items, path, index, height_limit, width_limit
        )
        stdscr.refresh()

        if DEBUG:
            stdscr.addstr(
                height_limit - 2,
                0,
                trim_string(
                    json.dumps(index_stack), width_limit, 0, False
                ),
            )

        # Listen for the next key press
        key = stdscr.getch()

        action = map_keys[key] if key in map_keys else Action.DEFAULT
        item = items[index]

        if DEBUG:
            stdscr.addstr(
                height_limit - 3,
                0,
                str(key),
                curses.A_REVERSE,
            )

        if action == Action.QUIT:
            return

        # Redraw the screen on terminal window resize
        if action == Action.REDRAW:
            return main(
                stdscr,
                path,
                index_stack,
                stack_position,
            )

        elif action == Action.UP:
            index -= 1

        elif action == Action.DOWN:
            index += 1

        elif action == Action.LEFT:
            return main(
                stdscr,
                os.path.abspath(os.path.join(path, os.pardir)),
                index_stack,
                stack_position - 1,
            )

        # Handle File Preview on macOS
        elif action == Action.PREVIEW and platform.system() == "Darwin":
            full_path = os.path.join(path, item[0])
            os.system(f"qlmanage -p {full_path} &> /dev/null &")

        elif (
            action == Action.RIGHT
            or action == Action.OPEN
            or action == Action.OPEN_DEFAULT
            or action == Action.EDIT
        ):
            full_path = os.path.join(path, item[0])
            if item[1]["is_dir"] and action == Action.RIGHT:
                return main(
                    stdscr, full_path, index_stack, stack_position + 1
                )
            elif not item[1]["is_dir"] and (
                action == Action.OPEN
                or (
                    (
                        action == Action.RIGHT
                        or action == Action.OPEN_DEFAULT
                    )
                    and is_binary(full_path)
                )
                or (Action.EDIT and "EDITOR" not in os.environ)
            ):
                open_file(full_path)
            else:
                # Write the result to a tempfile for the bash script to work
                # with
                with open(os.environ["tempfile"], "w") as file:
                    file.write(full_path)
            return


def entrypoint() -> None:
    """Program's entrypoint.

    raises:
        Exception: if "tempfile" environmental variable is not set
    """
    if "tempfile" not in os.environ:
        raise Exception('"tempfile" environmental variable is not set')

    stdscr = curses.initscr()
    try:
        config(stdscr)
        current_dir = sys.argv[1] if len(sys.argv) > 1 else os.getcwd()
        main(stdscr, current_dir)
    finally:
        terminate_curses(stdscr)


if __name__ == "__main__":
    entrypoint()
