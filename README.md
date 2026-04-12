# dir-explorer

Fast, opinionated `ls`-style directory listing written in Rust.

## Status

The Rust rewrite is the active implementation.

- no `ls`/`gls` subprocess wrapping
- custom extension/name coloring preserved from the Python version
- tree recursion mode and long metadata mode supported
- no icons by default and no icon rendering path in the current code

## Build

```bash
cargo build --release
```

Run directly:

```bash
cargo run -- [flags] [paths...]
```

## CLI

Defaults:

- `-A` semantics (almost-all)
- sort by version
- group directories first
- long-iso timestamps only
- default row: `size time name`

Flags:

- `-a`: include all entries
- `-A`: include almost all entries
- `-l`: detailed mode (`mode links owner group size time name`)
- `-0`: minimal mode (name only)
- `-H`: raw bytes for sizes
- `-r` / `--reverse`: reverse final sort order
- `-R` / `--recursive`: recursive tree listing (does not recurse into symlink targets)
- `-t`: sort by modified time
- `--sort=version|name|time|size|extension`
- `--color=always|auto|never`

## Notes on behavior

- `-l` uses octal permissions without a leading `0`.
- `--time-style` is intentionally not supported; output is always long-iso.
- External theme config is intentionally not supported.

## Legacy Python implementation

The pre-rewrite Python implementation is preserved at tag `v1`.

- tag: `v1`
- commit: `dcb4cea`
