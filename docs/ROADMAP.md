# Roadmap

* What tidymac does today, what comes next, and how each step is accepted.
* A milestone is only marked complete after its checks pass.
* Last updated September 24, 2026.

## Contents

1. [Status](#status)
2. [How The Work Is Organized](#how-the-work-is-organized)
3. [Complete](#complete)
4. [In Progress](#in-progress)
5. [Planned](#planned)
6. [P1 Release Checks](#p1-release-checks)

## Status

| Milestone | Goal | Status |
| --- | --- | --- |
| M0 | Workspace, licenses, formatting, linting, and CI | Complete |
| M1 | Scanner | In progress |
| M2 | Disk tab | Planned |
| M3 | Rules and dry runs | Planned |
| M4 | Cleanup to the Trash | Planned |
| M5 | P1 release | Planned |
| M6 | P1.5: large and old files, and app removal | Planned |
| M7 | P2: startup and background items, SSH, dotfiles, and optimization settings | Planned |

## How The Work Is Organized

* Work is split into milestones, M0 to M7.
* Each milestone leaves tidymac building and its tests passing.
* M0 to M5 make up P1. M6 is P1.5. M7 is P2.

## Complete

### M0: Foundation

* A Cargo workspace with `tidymac` and `tidymac-core`.
* MIT and Apache 2.0 licenses.
* CI checks formatting, Clippy, and tests, and builds for Apple silicon and Intel.

## In Progress

### M1: Scanner

* **Built:**
  1. `walk_directory` walks a folder without following symbolic links or leaving the disk.
  2. `allocated_size` works out the space a file uses on disk.
  3. Tests cover a plain folder, a symbolic link that is not followed, and a missing start folder.
* **Remaining:**
  1. Count hard links once.
  2. Build the folder tree.
  3. Send progress and errors while scanning.
  4. List any other disks it skipped, and mark incomplete scans.
* **Done when** tests with temporary folders cover links, files with empty parts, nested folders, permissions, and other disks.

## Planned

### M2: Disk Tab

* The column browser, sorting, moving around, progress, warnings, and the disk gauge.
* **Done when** the screen keeps responding during a scan, and marks incomplete scans.

### M3: Rules

* The TOML rule format, loading rules, the bundled rules, and dry run plans shown in the Clean tab.
* **Done when** every bundled rule loads and finds only allowed targets, without changing any file.

### M4: Cleanup

* The path check, checks for open apps, the path review, the question, and moving to the Trash.
* **Done when** the safety tests in [SAFETY.md](SAFETY.md#tests) pass, and Finder restores a test item.

### M5: P1 Release

* Programs for Apple silicon and Intel, Homebrew and Cargo packages, and steps for turning on Full Disk Access.
* **Done when** automated tests and the release checks below pass on both kinds of Mac.

### M6: P1.5

* Filters for large and old files, and the app removal review.
* **Done when** both use the existing scanner and safety checks, without widening any cleanup root.

### M7: P2

* The Startup tab, with background items.
* The SSH tab and the Dotfiles tab.
* The Settings tab: power mode, graphics switching, refresh rate, Game Mode guidance, what keeps the Mac awake, and wake settings.
* **Done when:**
  1. Every change saves its old state, and tests restore it.
  2. A refresh rate change that is not kept switches back.
  3. The `pmset` setting names are checked on real Macs, for each supported macOS version.

## P1 Release Checks

1. Compare a known scan with the sizes macOS reports.
2. Confirm that missing Full Disk Access gives clear warnings about skipped folders.
3. Read every path the bundled rules find.
4. Move a harmless item to the Trash and restore it with Finder.
5. Check the labels that explain size on disk and APFS limits.
6. Confirm tidymac shows every path and asks before changing anything.
