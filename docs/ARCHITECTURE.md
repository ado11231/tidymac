# Architecture

* How tidymac is built: its parts, how data moves between them, and what each file does.
* For people working on the code. To learn what tidymac does, read [FEATURES.md](FEATURES.md).

## Contents

1. [Overview](#overview)
2. [How The Code Is Organized](#how-the-code-is-organized)
3. [How Data Moves](#how-data-moves)
4. [The Scanner](#the-scanner)
5. [Cleanup](#cleanup)
6. [SSH And Dotfiles](#ssh-and-dotfiles)
7. [Settings And Startup](#settings-and-startup)
8. [Dependencies](#dependencies)
9. [Platform Support](#platform-support)
10. [File Reference](#file-reference)

## Overview

* tidymac is split into two crates. A crate is one Rust package.
  1. **`tidymac-core`** is the library. It scans, measures sizes, reads rules, checks paths, and plans cleanups. It never draws anything on screen.
  2. **`tidymac`** is the program you run. It holds the screens and keyboard input. It only accepts `--version` and `--help`, read from `std::env::args` without an argument parsing crate.
* `tidymac` uses `tidymac-core`. `tidymac-core` does not use `tidymac`.

## How The Code Is Organized

```text
tidymac/
├── Cargo.toml                 Workspace settings
├── crates/
│   ├── tidymac-core/          The library
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── scan.rs
│   │       ├── size.rs
│   │       ├── rules.rs
│   │       ├── safety.rs
│   │       └── clean.rs
│   └── tidymac/               The program
│       └── src/
│           └── main.rs
├── docs/
└── .github/workflows/ci.yml
```

* These are planned, but do not exist yet:

| Path | Will Hold |
| --- | --- |
| `crates/tidymac-core/tests/` | Tests that use the library from outside. |
| `crates/tidymac/src/ui/` | The terminal screens. |
| `rules/` | The bundled cleanup rules. |

## How Data Moves

### A Scan

```text
scan request
    -> walk the folders
    -> read each file's details
    -> build the folder tree
    -> show it on screen
```

### A Cleanup

```text
selected rules
    -> find the paths each rule matches
    -> check each path
    -> show every path to the user
    -> ask to confirm
    -> check each path again
    -> move it to the Trash
```

* Only a checked path, a `ValidatedPath`, can reach the code that moves files. See [SAFETY.md](SAFETY.md).

## The Scanner

* Starts in your home folder.
* Stays on the disk it started on, and lists any other disks it skipped.
* Lists symbolic links, but does not follow them. A symbolic link is a file that points to another path.
* Measures the space each file uses on disk as `st_blocks * 512`.
* Counts a file with several names, a hard link, only once. It spots them by `(st_dev, st_ino)`, two numbers that identify a file.
* Keeps every file and folder in one list, `Vec<Node>`. Each entry finds its parent by its place in the list.
* Sends each entry, its progress, and any errors to the screen while it runs.
* Marks the scan incomplete when a folder cannot be read.
* The sizes are estimates:
  1. Some files skip empty parts, so their length is larger than the space they use.
  2. On APFS, copied files can share space, and file details cannot show this.
  3. So the disk gauge uses the disk's own totals instead.

## Cleanup

* Most cleanup targets come from TOML rules built into the program.
* Your own rules load from `~/.config/tidymac/rules/`.
* Targets that need an app's own tool, such as Docker, use a `Cleaner` trait instead of a rule.
* All of them go through the same safety checks.

## SSH And Dotfiles

* Both change files where they are, so they do not use `ValidatedPath` or the Trash.
* A separate allow list in `tidymac-core` names every file they may change.
* Before each change, tidymac saves the old content or permissions.
* It writes the new content to a temporary file, then swaps it in, so a crash never leaves half a file.
* tidymac runs the usual tools instead of writing its own:

| Need | Tool |
| --- | --- |
| Key details and fingerprints | `ssh-keygen -l` |
| Agent keys | `ssh-add` |
| Removing known hosts | `ssh-keygen -R` |
| Checking a file for mistakes | `ssh -G`, `zsh -n`, `bash -n`, `git config --list --file` |
| Editing | Your editor, from `$VISUAL` or `$EDITOR` |

## Settings And Startup

* Both change Mac settings instead of files. tidymac saves the old value before each change, and undo puts it back.
* The allow list in `tidymac-core` names every setting and startup action they may use.
* A change that needs admin rights runs one command with `sudo`. tidymac itself never runs as root.
* tidymac runs the usual tools instead of writing its own:

| Need | Tool |
| --- | --- |
| Power mode, graphics switching, wake settings | `pmset` |
| Apps keeping the Mac awake | `pmset -g assertions` |
| Why the Mac woke overnight | `pmset -g log` |
| Refresh rates | CoreGraphics display modes |
| Background items | `sfltool dumpbtm` |
| Turning launch agents and daemons off or on | `launchctl` |

## Dependencies

* Versions live in `Cargo.toml`. Try each new crate in a small test before using it.

| Need | Crate | Status |
| --- | --- | --- |
| Walking folders | `walkdir` | In use |
| Temporary folders in tests | `tempfile` | In use |
| Terminal screens | `ratatui` with Crossterm | Candidate |
| Faster walking | `ignore` and `rayon` | Candidate |
| Reading TOML | `serde` and `toml` | Candidate |
| Building rules into the program | `include_dir` | Candidate |
| Checking for running apps | `sysinfo` or a macOS API | Candidate |
| Moving to the Trash | A crate or macOS API that works with Finder's Put Back | Candidate |
| Writing archives | `tar` and `flate2` | Candidate |
| Changing display modes | `core-graphics` | Candidate |
| Errors | `thiserror`, and `anyhow` in the `tidymac` crate if needed | Candidate |

## Platform Support

* macOS 13 or later, on Apple silicon and Intel.
* Not every disk uses APFS. If a disk does something tidymac does not support, it must stop with a clear message.
* The workspace does not allow `unsafe` code. macOS calls must go through a crate that handles that.

## File Reference

| File | Purpose |
| --- | --- |
| `Cargo.toml` | Lists the two crates, their shared version and license, and the lint rules. The rules turn on Clippy's strict checks and forbid `unsafe` code. |
| `crates/tidymac-core/Cargo.toml` | The library's dependencies: `walkdir`, and `tempfile` for tests. |
| `crates/tidymac-core/src/lib.rs` | The library's entry point. Makes the `scan` and `size` modules public. |
| `crates/tidymac-core/src/scan.rs` | `walk_directory` walks a folder without following links or leaving the disk. Has tests. |
| `crates/tidymac-core/src/size.rs` | `allocated_size` works out the space a file uses on disk. |
| `crates/tidymac-core/src/rules.rs` | Empty. Will read and check rules. |
| `crates/tidymac-core/src/safety.rs` | Empty. Will hold `validate_deletable` and `ValidatedPath`. |
| `crates/tidymac-core/src/clean.rs` | Empty. Will plan cleanups and move items to the Trash. |
| `crates/tidymac/Cargo.toml` | The program's dependencies: `tidymac-core`. |
| `crates/tidymac/src/main.rs` | The program's entry point. An empty `main` for now. Will open the terminal interface. |
| `.github/workflows/ci.yml` | Checks formatting, runs Clippy and the tests, and builds for Apple silicon and Intel. |
