# dir_explorer

CLI tools for quickly exploring the directory structure.

They were tested to work on macOS with ZSH and may work on other Unix/Linux
systems with slight modifications.

## `list_files.py`

A script that piggy bags on the output of `ls` by adding colors on top
of it. It allows to gray out unimportant files and to associate some
colors with certain file extensions.

### Configuration

You can customize the colors by [editing the color_definitions.py
file](#customize-color-definitions)

Additionally, if you are not on macOS, you would have to modify the
`ls` command as it has different parameters depending on the system.

To do so, open the `dir_explorer/list_files.py` file and modify the `ls`
variable to a command that should be used as a basis. The command should
have the flags necessary to output the files in the long format.

### Installation

Add this to your .bashrc/.zshrc file:

```bash
alias l="python3 ~/path/to/dir_explorer/dir_explorer/list_files.py"
```

Remember to change `~/path/to/dir_explorer/` to an actual path.

### Usage

Then you can just press `l` in the terminal, hit ENTER, and look in awe at
all the colors!

And just like with regular `ls` command, you can provide a directory name as the
second parameter. You can even use a wildcard or multiple directories. Example:

```bash
l ~/Downloads/*.png ~/Documents/
```

Or display files recursively:

```bash
l -R ~/Downloads
```

## `list_view.py`

`list_view.py` aims to provide an experience similar to Finder's list
view right in your terminal. It is most useful for when you need to
explore several directories and quickly move between levels as you
search for the thing you are looking for.

### Configuring

You can customize the colors by [editing the color_definitions.py
file](#customize-color-definitions).

Additionally, you can set the environmental variable `EDITOR` to a CLI editor
you want to use for opening text files

### Installing

Create a virtual environment:

```bash
python -m venv venv
```

If you choose to omit the creation of virtual environment, then you would have
to change the location of python executable in `dir_explorer/list_view`.

Run pip install:

```bash
./venv/bin/pip install -r requirements.txt
```

Then, add this to your .zshrc file:

```bash
alias f="/bin/zsh ~/path/to/dir_explorer/dir_explorer/list_view"
```

Remember to change `~/path/to/dir_explorer/` to an actual path and `/bin/zsh` to
your preferred shell.

### Running

Then you can call `f` or `f <some path>` to display a directory in a list
view.

Controls:

* `Up / Down Arrow` - navigate Up / Down
* `Left / Right Arrow` - open parent / child directory

Instead of the arrow keys, you can also use `WSAD` or `HJKL`.

Next, you can `cd` to the selected directory by pressing the `Space key`.

For opening files, you can use `E` to open it in the text editor, `O` for
opening it in the default application, or Space bar to use the best application
for that file type.

Additionally, macOS users can press the `P` key to quickly preview the file or
directory.

## Customize color definitions

After that, feel free to modify the `dir_explorer/color_definitions.py`
as you see fit. That file defines the colors to use for each file type.

For color codes in modern terminals, see [this nice
article](https://tforgione.fr/posts/ansi-escape-codes/).

The `color_definitions.py` file contains a dictionary
`color_definitions` that associates a human-friendly label with one of
the 255 colors.

Then, in `known_file_extensions`, file extensions are assigned a certain
color.

Also, for extensions like `.zshrc`, `.eslitrc` and other `rc` files,
there is a useful `known_file_extension_endings` dictionary that defines
colors for files whose extensions end with a given string. Finally,
there is a `known_file_names` dictionary for when you need to associate
an entire file name with a color.
