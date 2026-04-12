# Better `ls` rewrite notes

## Goal

Replace the current Python wrapper with a native Rust implementation that:

- keeps the current opinionated defaults
- preserves the parts of GNU `ls` output that are comfortable to scan
- keeps custom filename / extension coloring from this repo
- avoids shelling out to `ls` entirely
- stays fast enough to use as the default directory listing command

The current Python codebase effectively has two separate tools:

- `list_files.py`: the actual `ls` wrapper to replace
- `list_view.py`: a separate TUI browser, not part of the rewrite target

This Rust rewrite should target the behavior of `list_files.py`, while reusing the color ideas from `color_definitions.py` / `colorize_name.py`.

---

## What the current Python wrapper actually does

Today the wrapper runs GNU `ls` / `gls` with these defaults:

- `--color`
- `--group-directories-first`
- `-Ahl`
- `--time-style=long-iso`
- `--sort=version`

Then it post-processes the long-format output:

- finds the split between metadata columns and filename column by looking for shared spaces
- dims the metadata portion
- preserves `ls` coloring for directories / symlinks / executables / other special files
- applies repo-specific filename coloring only to regular non-executable files
- adds a `▸` marker before some non-regular entries

The important takeaway is that the value of the tool is **not** just listing files. It is the combination of:

1. opinionated defaults
2. low-noise long view
3. custom filename coloring by exact name / extension / suffix
4. visually de-emphasized metadata

The Rust version should preserve those four things.

---

## Product definition for the Rust version

### Default behavior

Running the tool with no flags should behave roughly like:

- almost-all mode (`-A` semantics, not `-a`)
- long listing
- human-readable sizes
- long ISO timestamp
- version-aware sorting
- directories grouped first
- colors enabled by default
- compact long output: no user, no group, no link count by default
- no `total` row

Suggested mental model:

> “GNU `ls` long mode, but quieter, colored more intentionally, and implemented directly.”

### Default visible columns

Default long row should likely be:

- file type / permission summary (condensed)
- size
- timestamp
- colored filename

By default, omit:

- owner
- group
- hard link count

And provide one opt-in flag to restore the full classic long metadata block.

---

## Recommended CLI behavior

### Keep / support

- `-a` = show all, including `.` and `..`
- `-A` = show almost all
- `-l` = long view
- `-h` = human-readable sizes
- `-H` or `--bytes` = disable human-readable formatting and print raw size
- `-r` / `--reverse`
- `-t` = sort by modified time
- `--sort=version|name|time|size|extension`
- `-R` / `--recursive`
- `--color=always|auto|never`
- `--time-style=long-iso|iso|relative` (even if only `long-iso` is initially implemented)

### Opinionated defaults

- default sort: `version`
- default grouping: directories first
- default visibility: `almost all`
- default time style: `long-iso`
- default size style: human-readable
- default color: `auto` unless disabled via env var

### Add one “classic long” escape hatch

Add a single flag that restores the noisy `ls -l` style metadata, for example:

- `--classic-long`

That mode would show:

- permissions
- link count
- owner
- group
- size
- timestamp
- name

For permissions, prefer octal if you want the compact mode to stay consistent, but for classic mode it may still be worth supporting symbolic permissions too.

---

## Output design recommendation

### 1. Keep metadata first

The Python note about putting filenames first is correct: it is probably worse for scanning in a dense terminal listing.

Recommended decision:

- keep metadata first
- dim metadata
- keep filename last and visually prominent

That preserves alignment and makes long filenames harmless.

### 2. Use fixed logical columns

Do not emulate the Python spacer-detection trick. In Rust, compute columns directly from structured file metadata.

Recommended long row layout:

`[perm/type?] [size] [time] [name]`

Where:

- `[perm/type?]` is compact and dim
- `[size]` is right-aligned and dim
- `[time]` is dim
- `[name]` contains the main colors

Optional full mode:

`[mode] [links] [owner] [group] [size] [time] [name]`

### 3. No `total` line

Keep omitting the `total` line by default.

### 4. Preserve arrow / special marker only if it still helps

The current wrapper adds `▸` to some non-regular entries after dimmed metadata. That is a post-processing artifact from piggybacking on `ls` output.

For the Rust version, treat this as optional, not required.

Recommendation:

- skip `▸` in v1 unless it clearly improves scanability
- rely on color + file type styling instead

---

## File type handling plan

The Rust implementation should classify entries explicitly instead of relying on `ls` colors.

At minimum support these file kinds:

- regular file
- directory
- symlink
- pipe / FIFO
- socket
- block device
- character device
- executable regular file
- broken symlink
- other / unknown special

Nice to have:

- mount point detection

Notes:

- On Unix, use metadata / file mode bits from `std::os::unix::fs::MetadataExt`.
- Symlinks should be read with `symlink_metadata()` first, then optionally resolved with `metadata()`.
- Broken symlinks should render distinctly.
- Executable should mean a regular file with any execute bit set.

---

## Color system plan

### Preserve the current repo-specific coloring model

Port the current Python color logic directly:

- exact filename matches
- exact extension matches
- extension-ending matches
- named palette entries + raw ANSI colors

That logic is a core differentiator from `eza`.

### Split styling into two layers

#### Layer 1: file-kind styling

Used for:

- directories
- symlinks
- executables
- devices
- sockets
- pipes
- broken symlinks

These should follow GNU `ls`-style expectations unless you intentionally want a different palette.

#### Layer 2: regular-file name coloring

Used only for regular, non-executable files.

For these files:

- preserve basename in default foreground
- color only the extension / matched segment, as the current Python code does

This is one of the clearest design requirements for the rewrite.

### Theme/config format

Recommended approach:

- keep a built-in default theme in Rust
- support loading an external config later
- initially port `color_definitions.py` into a Rust config module rather than inventing a new format immediately

Do **not** block v1 on designing a perfect external theme format.

---

## Escaping and name rendering

One of the explicit goals is to “follow same escaping printing as `ls`”.

This needs to be treated as a real feature area, not an afterthought.

Recommended v1 rules:

- print UTF-8 names as-is when valid and terminal-safe
- escape control characters predictably
- do not let ANSI escapes from filenames pass through unescaped
- handle tabs / newlines / carriage returns safely

Suggested implementation strategy:

- maintain an internal `display_name(entry_name)` function responsible for all escaping
- keep that function separate from coloring
- add tests for weird filenames

This will likely be one of the hardest parts to match with `ls` exactly, so it should be isolated behind a dedicated formatter module.

---

## Sorting and grouping plan

### Default sort

Default to version-aware sort, matching the current wrapper.

Recommended rules:

1. directories first
2. then apply chosen sort key within each group
3. stable tie-breaker: raw filename

### Sort modes to support in v1

- `name`
- `version` (default)
- `time`
- `size`
- `extension`

### Reverse

`--reverse` should invert the final ordering after grouping + sorting logic is applied consistently.

---

## Recursive mode plan

You want recursive mode to behave like a tree rather than repeated `ls -R` sections. That is a good reason to avoid wrapping `ls`.

Recommended recursive behavior:

- `-R` / `--recursive` switches output into tree mode
- directories are shown once, nested inline
- indentation / tree guides show structure
- sort/group rules remain the same at every depth

Example shape:

```text
src/
├── main.rs
├── render/
│   └── mod.rs
└── theme/
    └── default.rs
```

Questions to settle during implementation:

- whether tree mode still shows long metadata for every entry
- whether directory headers are repeated when multiple roots are passed
- how symlinked directories behave during recursion

Recommended v1 decisions:

- keep long metadata in recursive mode unless a dedicated tree-only view is later added
- do not follow symlinked directories by default
- when multiple roots are passed, print a labeled block per root

---

## Multi-path and glob behavior

Because the old wrapper passed arguments through to the shell command, the shell handled glob expansion before Python saw them.

The Rust tool should support:

- one or more paths as positional arguments
- files and directories mixed together
- shell-expanded globs naturally (shell still expands them before the binary runs)

If multiple paths are passed:

- print a single listing if all are files
- print labeled sections if any path is a directory

---

## Cross-platform scope recommendation

Since the current tool already has macOS and Linux assumptions, v1 should target Unix first.

Recommended scope:

- v1: macOS + Linux
- no Windows support initially

This keeps file mode handling, device detection, and terminal behavior straightforward.

---

## Proposed Rust architecture

Recommended crate/module split:

- `main.rs` — CLI parsing and top-level execution
- `cli.rs` — flags, defaults, option normalization
- `fs.rs` — reading directories, metadata collection, symlink handling
- `model.rs` — `Entry`, `FileKind`, `RenderOptions`, `SortMode`
- `sort.rs` — comparators and grouping
- `theme.rs` — palette + file match rules
- `color.rs` — ANSI style helpers
- `format_name.rs` — escaping + name segmentation
- `format_meta.rs` — permissions, size, timestamp formatting
- `render.rs` — row rendering for flat and recursive modes

Core data model:

- `Entry`
  - raw path
  - display name
  - file kind
  - permissions / mode bits
  - size
  - modified time
  - symlink target if applicable
  - maybe resolved target kind for symlinks

Important design rule:

- collect metadata into structured objects first
- sort second
- render last

Do not intertwine filesystem access with string rendering.

---

## Suggested crates

Likely useful:

- `clap` for CLI parsing
- `owo-colors` or `anstyle` for ANSI styling
- `terminal_size` or `crossterm` only if width-awareness becomes necessary
- `humansize` or a tiny custom formatter for sizes
- `chrono` or `time` for timestamp formatting
- `natord` or a custom natural/version sort implementation
- `anyhow` for CLI-level error handling

Possible preference:

- keep dependencies light unless a crate clearly saves time

---

## Implementation phases

### Phase 1: basic direct replacement

Build a flat non-recursive listing with:

- path argument handling
- hidden file filtering (`-A` / `-a`)
- metadata collection
- version sort
- directories first
- long ISO time
- human-readable sizes
- regular file / dir / symlink / executable coloring

This phase should be enough to replace the shell wrapper for daily use.

### Phase 2: custom filename coloring parity

Port:

- exact filename rules
- exact extension rules
- extension-ending rules
- dimmed metadata rendering

At this point the Rust output should feel like the current Python tool, not just like another `ls` clone.

### Phase 3: edge-case parity

Add:

- sockets / pipes / devices
- broken symlink rendering
- `--reverse`
- `-t`
- `--classic-long`
- robust escaping behavior

### Phase 4: recursive tree mode

Add:

- `-R` tree rendering
- depth-first traversal
- loop protection if following symlinks is ever added

### Phase 5: polish

Add optional:

- external config file
- additional time styles
- width-aware truncation
- icons only if explicitly enabled

---

## Testing plan

The Rust rewrite should include snapshot-style tests for output formatting.

Test categories:

### File classification

- regular file
- executable file
- directory
- symlink to file
- symlink to directory
- broken symlink
- pipe / socket / device where testable

### Sorting

- version sort (`file2` before `file10`)
- reverse sort
- directories-first interactions
- time sort

### Visibility

- default almost-all behavior
- `-a`
- `-A`

### Formatting

- human-readable vs raw bytes
- long-iso timestamp
- classic-long mode
- dim metadata + colored filename

### Weird names

- unicode
- spaces
- tabs
- newlines
- ANSI escape bytes in filenames
- leading dots

---

## Open decisions

These are the only design questions that still seem worth deciding before implementation:

1. Should default compact mode show octal permissions, symbolic permissions, or neither?
2. Should recursive tree mode use the same long-row format, or a more compact tree-specific row?
3. Should symlink display include `name -> target` in default mode?
4. Should color config remain hard-coded initially, or move immediately to a file format like TOML / YAML?

My recommendation:

1. compact mode shows a minimal type/permission indicator, not full symbolic permissions
2. recursive mode keeps the same long-row format in v1
3. symlinks should show `-> target`
4. keep colors built-in first, externalize later

---

## Final recommendation

Build the Rust tool as a **direct structured renderer**, not as an `ls` emulator and not as a post-processor.

The main thing to preserve from the current Python implementation is:

- opinionated defaults
- dim metadata
- custom extension/name coloring
- low-noise long layout

The main thing to improve over the Python version is:

- no shelling out
- correct structured metadata handling
- better special-file support
- tree-style recursion
- predictable escaping

If done well, this should end up feeling like:

> a fast, opinionated, readable Unix directory lister with extension-aware coloring — not just “another `ls` clone”.
