# dir-explorer

Clean terminal directory listing.

Better defaults than `ls`. Much cleaner than `eza`. Faster than either of them.

Features:

- colorized output based on file name or extension
- directories grouped first
- natural sorting
- long ISO timestamps

## What it looks like

Efficient default view:

```log
> l
   4.7k 2026-04-12 16:24 cli.rs
    964 2026-04-12 14:08 color.rs
        2026-04-12 14:27 src
```

Long view:

```log
> l -l
 755 maxpatiiuk staff 2026-04-12 14:27 src
 644 maxpatiiuk staff 4.7k 2026-04-12 16:24 cli.rs
```

Recursive view:

```log
> l -R
        2026-04-12 14:27 src
   4.7k 2026-04-12 16:24 │   ├── cli.rs
    964 2026-04-12 14:08 │   ├── color.rs
```

> Recursive view skips common black-hole directories such as `.git` and `node_modules`.

## Build

```bash
cargo build --release
```

Run directly:

```bash
cargo run -- [flags] [paths...]
```

## Install

Suggested `~/.zshrc` setup:

```sh
alias l="~/g/dir-explorer/target/release/dir-explorer"
alias ll="l -l"
alias l0="l -0"
alias lr="l -R"
alias lt="l -t"
alias lS="l --sort=size"

function cl() {
  local dir="${*:-$HOME}"
  builtin cd "$dir" && l
}
compdef cl=cd
```

## CLI help

For the current defaults and full flag list, see [CLI_HELP.txt](./CLI_HELP.txt).

Or run `l -h`.

## Customization

To tweak colors and built-in filename or directory rules, edit [src/theme.rs](src/theme.rs).

To tweak home-directory-specific hiding and gray-out behavior, edit [src/home_overrides.rs](src/home_overrides.rs).

That file defines:

- color names and ANSI mappings
- file extension color rules
- exact filename color rules
- directory color rules
- black-hole directories skipped by recursive view

## Prior implementation

The older Python-based version is preserved as tag `v1` (`dcb4cea`). Port from Python to Rust was LLM-assisted.
