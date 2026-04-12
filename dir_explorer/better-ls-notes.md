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
- human-readable sizes
- long ISO timestamp
- version-aware sorting
- directories grouped first
- colors enabled by default
- no `total` row

Suggested mental model:

> “A quieter, faster `ls` with good defaults and custom filename coloring.”

### Default visible columns

Default row should be:

- size
- timestamp
- colored filename

Do **not** show file type or permission summary by default.

Rationale:

- the important practical distinctions are already covered by colors for directories, symlinks, and executables
- removing the mode column makes the default view lighter and faster to scan
- permissions matter mostly when explicitly debugging permissions, which fits `-l`

Add a short mode for later aliasing:

- `-0` = name-focused compact mode with no size and no timestamp

---

## Recommended CLI behavior

### Keep / support

- `-a` = show all, including `.` and `..`
- `-A` = show almost all
- `-l` = long view
- `-0` = omit size and timestamp
- `-H` = disable human-readable formatting and print raw size
- `-r` / `--reverse`
- `-t` = sort by modified time
- `--sort=version|name|time|size|extension`
- `-R` / `--recursive`
- `--color=always|auto|never`

### Opinionated defaults

- default sort: `version`
- default grouping: directories first
- default visibility: `almost all`
- timestamp style: always `long-iso`
- default size style: human-readable
- default color: `auto` unless disabled via env var

### Define `-l` as the detailed mode

Instead of a separate classic flag, let `-l` be the detailed mode.

`-l` should show:

- permissions
- link count
- owner
- group
- size
- timestamp
- name

For permissions, use **octal without a leading `0`**.

Example:

- `755` instead of `0755`

This keeps the detailed mode compact while still being more immediately useful than symbolic permissions for common shell work.

---

## Output design recommendation

### 1. Keep metadata first

The Python note about putting filenames first is correct: it is probably worse for scanning in a dense terminal listing.

Recommended decision:

- keep metadata first
- dim metadata when metadata is shown
- keep filename last and visually prominent

That preserves alignment and makes long filenames harmless.

### 2. Use fixed logical columns

Do not emulate the Python spacer-detection trick. In Rust, compute columns directly from structured file metadata.

Recommended long row layout:

Default mode:

`[size] [time] [name]`

Where:

- `[size]` is right-aligned and dim
- `[time]` is dim
- `[name]` contains the main colors

`-l` mode:

`[octal_mode] [links] [owner] [group] [size] [time] [name]`

`-0` mode:

`[name]`

### 3. No `total` line

Keep omitting the `total` line by default.

### 4. Do not include the arrow marker

Do not carry over the `▸` marker.

Recommendation:

- no extra marker in any mode
- rely on color and optional `-l` metadata instead

---

## File type handling plan

The Rust implementation should classify entries explicitly instead of relying on `ls` colors.

At minimum, classify these file kinds:

- regular file
- executable regular file
- directory
- symlink
- broken symlink

Only classify edge-case Unix kinds when they are a trivial byproduct of already-collected metadata.

Policy:

- do not add separate syscalls just to distinguish edge-case file kinds
- if one metadata read already exposes kind bits, use them
- otherwise fold into `other / unknown special`

### Which kinds matter most in practice

If the goal is a simpler and faster v1, the most valuable distinctions are:

1. `directory`
   - why care: navigation target; should sort first and have strong color
2. `symlink`
   - why care: may point elsewhere, may be broken, and should usually render with `name -> target`
3. `broken symlink`
   - why care: likely actionable problem; deserves distinct styling
4. `executable regular file`
   - why care: permission bit changes behavior directly; color replaces the need to always show mode bits
5. `regular file`
   - why care: gets custom extension/name coloring

These five kinds cover almost all day-to-day use.

The rest are edge-case Unix kinds:

- `pipe / FIFO`
  - why care: usually indicates shell plumbing or IPC; rare in normal project trees
- `socket`
  - why care: often indicates a live service or local IPC endpoint
- `block device`
  - why care: important in system directories, rarely relevant in normal repo browsing
- `character device`
  - why care: same as above; mostly system-facing
- `other / unknown special`
  - why care: defensive fallback so weird files still render safely

Implementation rule for these kinds:

- include them only when available from the same metadata used for regular file-kind detection
- no additional per-entry syscall round-trips for deeper classification

### Simpler implementation recommendation

For v1, fully optimize for these paths:

- regular file
- executable regular file
- directory
- symlink / broken symlink

Then detect other Unix special kinds correctly but keep their rendering minimal.

That gives correct behavior without spending disproportionate effort on rare file types.

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
- port `color_definitions.py` into a Rust config module
- do not support external theme config

This keeps the rewrite smaller and avoids spending time on config parsing rather than rendering behavior.

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

This matches your desired behavior:

- `--recursive` still shows long metadata when `-l` is enabled
- recursive traversal must not recurse into symlinks

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

Recommended file/module split in this same folder:

- `main.rs` — CLI parsing and top-level execution
- `cli.rs` — flags, defaults, option normalization
- `fs.rs` — reading directories, metadata collection, symlink handling
- `model.rs` — `Entry`, `FileKind`, `RenderOptions`, `SortMode`
- `sort.rs` — comparators and grouping
- `theme.rs` — built-in palette + file match rules
- `color.rs` — ANSI style helpers
- `format_name.rs` — escaping + name segmentation
- `format_meta.rs` — size, timestamp, octal mode formatting
- `render.rs` — row rendering for flat and recursive modes

These Rust files should live alongside the current Python files in this `dir_explorer` folder, since you plan to delete the Python implementation after parity is reached.

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

It is practical to build this tool with **zero external crates**.

Because the scope is intentionally narrow, the standard library already covers most needs:

- directory reading: `std::fs`
- metadata: `std::fs` + `std::os::unix::fs::MetadataExt`
- symlink handling: `symlink_metadata()` / `read_link()`
- argument parsing: `std::env::args()`
- ANSI escape emission: plain strings
- path handling: `std::path`

For this project, zero dependencies is a realistic default, not a stunt.

### Strong recommendation: start with no external crates

Why this is practical here:

- CLI surface is small and stable
- colors are simple ANSI sequences, not complex terminal abstractions
- no Windows support means fewer portability abstractions are needed
- no terminal width handling is needed
- no external theme parsing is needed
- long-iso is the only timestamp style, which avoids a large date-formatting surface

### Where crates may still be justified

#### `clap`

Useful only if you want polished help text, automatic validation, and easier flag maintenance.

Why you might skip it:

- the flags are simple enough to parse manually
- manual parsing keeps startup lean and avoids a large dependency tree

Recommendation:

- v1: parse arguments manually
- add `clap` later only if the CLI grows substantially

#### `time`

Useful if you want robust local-time formatting without shelling out or writing unsafe FFI.

Why it may be justified:

- formatting filesystem times as local `YYYY-MM-DD HH:MM` is annoying with std alone
- a small, focused time crate can reduce custom date code and avoid platform-specific hacks

Why you might skip it:

- if exact long-iso formatting can be implemented with a tiny amount of platform-specific code
- if you are willing to keep timestamp formatting minimal at first

Recommendation:

- this is the one external crate most likely worth using

#### Natural/version sort crate

Useful if you want battle-tested version-aware ordering immediately.

Why you might skip it:

- a small custom comparator for ASCII-ish filenames may be enough
- your desired sort behavior is narrower than a full locale-aware natural sort implementation

Recommendation:

- try a custom comparator first
- only add a crate if edge cases become annoying

#### Error handling crate such as `anyhow`

Useful for ergonomic error propagation in a CLI.

Why you might skip it:

- the binary is small
- `Result<T, String>` or small custom error enums are sufficient

Recommendation:

- not necessary for v1

### Crates not needed

Do **not** add crates for:

- terminal sizing
- config parsing
- theming systems
- cross-platform abstractions for Windows

### Final crates recommendation

Best default plan:

- zero external crates if possible
- optionally one time-formatting crate if local timestamp formatting is too annoying with std alone

That approach best matches the project goals: fast startup, small surface area, and full control over rendering.

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
- default row format: `[size] [time] [name]`
- `-0` row format: `[name]`

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
- `-l` with octal permissions, link count, owner, group, size, time, and name
- robust escaping behavior

### Phase 4: recursive tree mode

Add:

- `-R` tree rendering
- depth-first traversal
- loop protection if following symlinks is ever added

### Phase 5: polish

Add optional:

- internal cleanup / refactoring after parity
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
- `-l` mode
- `-0` mode
- dim metadata + colored filename

### Weird names

- unicode
- spaces
- tabs
- newlines
- ANSI escape bytes in filenames
- leading dots

---

## Settled decisions

These points are now decided:

1. default mode shows no file type / permission summary
2. `-l` is the detailed mode and uses octal permissions without a leading `0`
3. `-0` is the minimal mode and omits size and timestamp
4. timestamp style is always long-iso; no `--time-style`
5. there is no arrow marker
6. recursive mode does not recurse into symlink targets
7. recursive mode can still show long metadata when `-l` is enabled
8. external theme config is out of scope
9. no Windows support

---

## Final recommendation

Build the Rust tool as a **direct structured renderer**, not as an `ls` emulator and not as a post-processor.

The main thing to preserve from the current Python implementation is:

- opinionated defaults
- dim metadata when metadata is shown
- custom extension/name coloring
- low-noise default layout

The main thing to improve over the Python version is:

- no shelling out
- correct structured metadata handling
- better special-file support
- tree-style recursion
- predictable escaping
- explicit support for `-l` and `-0`

If done well, this should end up feeling like:

> a fast, opinionated, readable Unix directory lister with extension-aware coloring — quieter than `ls -l`, but still able to become detailed on demand.
