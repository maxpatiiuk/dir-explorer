considering improvements to ls:

- rewrite to rust for less latency
- support --reverse
- --sort=version by default
- --color unless disabled by env var
- --group-directories-first always
- -Ahl by default (--almost-all, not -all)
- --time-style=long-iso by default
- support -t shorthand. -a
- while -h is the default, have a flag to disable that. -H?
- Don't show permissions, group and user and links by default, but have a single flag to add them all back. for permissions use octal
- follow gls colors for executables and symlinks
- while we could show file name first (most important part first) and them dim metadata, some issues:
  - long file names
  - might be harder to scan visually as you no longer can scan just the endings of lines
  - metadata far away if one file name is longer
- don't show "total" row
- support --recursive but it should print as a tree rather than recursive call
- follow same escaping printing as ls
- any edge cases or special files that ls handles? what are these?
  normal
  directory
  symlink
  pipe
  block_device
  char_device
  socket
  special
  executable
  mount_point
