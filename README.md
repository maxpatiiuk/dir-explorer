# dir_explorer

CLI tools for quickly exploring the direcory structure.

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

### Usage

Add this to your .bashrc/.zshrc file:

```
alias l="python3 ~/path/to/dir_explorer/dir_explorer/list_files.py"
```

Then you can just type `l` in the termial, hit ENTER, and look in awe at
all the colors!

## `list_view.py`

**This command is still under development**

`list_view.py` aims to provide an experience similar to Finder's list
view right in your terminal. It is most useful for when you need to
explore several directories and quickly move between levels as you
search for the thing you are looking for.

### Config

You can customize the colors by [editing the color_definitions.py
file](#customize-color-definitions)

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
