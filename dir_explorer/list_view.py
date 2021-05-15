import os
import sys
import curses
import platform
import subprocess
import json
from typing import List, Tuple, TypedDict


home_directory = os.path.expanduser("~")

map_keys: dict[str, str] = {
    "KEY_UP": "up",
    "KEY_DOWN": "down",
    "KEY_LEFT": "left",
    "KEY_RIGHT": "right",
    "W": "up",
    "S": "down",
    "A": "left",
    "D": "right",
    "Q": "quit",
    "K": "up",
    "J": "down",
    "H": "left",
    "L": "right",
}


class ItemProps(TypedDict):
    is_dir: bool


def get_items(path: str) -> List[Tuple[str, ItemProps]]:

    items = [
        (item, dict(is_dir=os.path.isdir(os.path.join(path, item))))
        for item in os.listdir(path)
    ]

    items.sort(key=lambda item: str(not item[1]["is_dir"]) + item[0])

    return items


def display_path(
    stdscr,
    items: List[Tuple[str, ItemProps]],
    path: str,
    selected: int,
    height_limit: int,
) -> int:

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
        relative_path + " " * (max_length - len(relative_path)),
        curses.A_REVERSE,
    )

    if len(items) + 1 >= height_limit:
        lines_to_show = int((height_limit - 1) / 2)
        # last_item = selected+lines_to_show
        # first_item = selected-lines_to_show
        # if first_item < 0:
        #     first_item = 0
        # total_items = items[first_item:last_item]
        # if len(total_items)+1 < height_limit:

        previous_items = items[selected - lines_to_show : selected]
        current_item = items[selected]
        next_items = items[selected : selected + lines_to_show]
        total_items = [*previous_items, current_item, *next_items]
        # if len(total_items)+1 < height_limit:
        #     if len(previous_items) < lines_to_show:
        #         total_items = [*total_items, items[selected+lines_to_show:selected+lines_to_show*2]]
        items = total_items

    for index, (item, item_props) in enumerate(items):
        stdscr.addstr(
            index + 1,
            0,
            item + " " * (max_length - len(item)),
            curses.color_pair(
                1
                if index == selected
                else 2
                if item_props["is_dir"]
                else 0
            ),
        )

    return len(items)


def terminate_curses(stdscr):
    curses.nocbreak()
    curses.echo()
    curses.endwin()
    stdscr.keypad(False)


def config(stdscr):

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


def open_file(file):
    if platform.system() == "Darwin":  # macOS
        subprocess.call(("open", file))
    elif platform.system() == "Windows":  # Windows
        os.startfile(file)
    else:  # linux variants
        subprocess.call(("xdg-open", file))


def menu(stdscr, path, index_stack: List[int] = None) -> None:

    height_limit, width_limit = stdscr.getmaxyx()
    stdscr.clear()

    if not index_stack:
        index_stack = []

    index = index_stack.pop() if index_stack else 0
    max_index = 1
    items = get_items(path)

    stdscr.addstr(height_limit - 2, 0, json.dumps(index_stack))
    stdscr.addstr(height_limit - 1, 0, path)

    while True:

        index = index % (max_index if max_index else 1)
        max_index = display_path(
            stdscr, items, path, index, height_limit
        )

        key = stdscr.getkey().upper()

        action = map_keys[key] if key in map_keys else "else"
        item = items[index]

        if action == "quit":
            return

        elif action == "up":
            index -= 1

        elif action == "down":
            index += 1

        elif action == "left":
            menu(
                stdscr,
                os.path.abspath(os.path.join(path, os.pardir)),
                index_stack[:-1],
            )
            return

        else:
            full_path = os.path.join(path, item[0])
            if item[1]["is_dir"]:
                if action == "right":
                    menu(stdscr, full_path, [*index_stack, index])
                    return
                else:
                    terminate_curses(stdscr)
                    print(full_path)
            else:
                open_file(full_path)
            return

    # stdscr.refresh()


def main():
    stdscr = curses.initscr()
    try:
        config(stdscr)
        current_dir = sys.argv[1] if len(sys.argv) > 1 else os.getcwd()
        menu(stdscr, current_dir)
    finally:
        terminate_curses(stdscr)


if __name__ == "__main__":
    main()
