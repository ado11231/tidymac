# Features and interface

This document defines the product scope and user interface. See [Architecture and
roadmap](ARCHITECTURE.md) for build order and [Safety and cleanup rules](SAFETY.md) for file
operations.

| Phase | Meaning |
|---|---|
| `P1` | First release |
| `P1.5` | Planned follow-up |
| `P2` | Planned, but not fully designed |

## Feature scope

### Disk visibility

| Phase | Feature |
|---|---|
| `P1` | Home-directory scan with an optional narrower root |
| `P1` | Allocated size, hard-link deduplication, and relative size bars |
| `P1` | Column browser sorted by size, name, or file count |
| `P1` | Volume disk gauge, access warnings, and streaming progress |
| `P2` | Optional treemap |

Directory totals estimate allocation. APFS clones may share blocks, so totals cannot predict exact
reclaimed space.

### Cleanup

| Phase | Feature |
|---|---|
| `P1` | Dry run, full path review, confirmation, and Trash destination |
| `P1` | Safe, caution, and expert risk tiers |
| `P1` | Running-application checks, age filters, and selection totals |
| `P1` | Bundled TOML rules and user rules |
| `P1` | Xcode, simulator, package-manager, application, browser, log, saved-state, Mail, and Docker targets |

P1 cannot permanently delete files or empty the Trash.

### Later phases

| Phase | Area | Scope |
|---|---|---|
| `P1.5` | Large and old files | Filter by size or modification time and open results in the Disk tab |
| `P1.5` | Applications | Review related files, leave possible user data unselected, warn about shared containers, and refuse removal while the app runs |
| `P2` | Startup | Inspect login items and launch services with reversible disable actions |
| `P2` | SSH | Show public configuration, key metadata, agent state, known hosts, and permission issues |
| `P2` | Settings | Apply reviewed settings, record old values, and support full revert |

The SSH view must never display private key contents, delete keys, or edit SSH configuration.
File age must mean modification time, not when a file was last opened. If macOS login-item data
cannot be read, the Startup tab must mark its results incomplete.

### Shared features

P1 includes keyboard navigation, contextual help, command-line and JSON output, one standalone
binary, Homebrew and Cargo installation, and Apple silicon and Intel support. Persistent
configuration is planned for P2.

### Excluded

- Disabling System Integrity Protection
- Removing Apple system applications
- Modifying `nvram`
- Automatic duplicate-file deletion
- Memory purging
- One-key cleanup without review
- Permanent deletion in P1

## Terminal interface

The Dashboard opens first and shows volume space, scan completeness, cleanup categories, and links
to the Clean tab. Directory estimates and volume statistics need separate labels because APFS
clones, snapshots, and purgeable data can make them differ.

Global keys:

| Key | Action |
|---|---|
| `Tab` / `Shift-Tab` | Next or previous tab |
| `?` | Help for the current screen |
| `q` | Quit when no dialog is open |

Arrow keys work throughout. Vim keys `h`, `j`, `k`, and `l` work outside text input.

### Disk tab

The Disk tab is a column browser rooted at the user's home directory by default.

| Key | Action |
|---|---|
| `Right` or `Enter` | Open the selected folder |
| `Left` | Return to the parent |
| `Up` / `Down` | Move selection |
| `s` | Change sort order |
| `d` | Create a cleanup plan for the selected item |
| `g` / `G` | Jump to the first or last row |

Cleanup from this tab uses the standard validation, review, and confirmation flow.

### Clean tab

Each row shows a rule's name, risk tier, item count, and estimated allocated size.

| Key | Action |
|---|---|
| `Space` | Select or clear an available rule |
| `Enter` | Review resolved paths |
| `?` | Explain the selected rule and tier |

Safe rules start selected. Caution rules require manual selection. Expert rules require typed
confirmation. A rule is unavailable while any application it requires to be closed is running.

The cleanup flow is fixed:

1. Select rules.
2. Review every resolved path.
3. Confirm the count, estimated size, and Trash destination.
4. Revalidate paths and move approved items to the Trash.

The terminal interface cannot skip path review.

## Command-line interface

```sh
# Scan home or a narrower path
tidymac scan
tidymac scan ~/Library
tidymac scan ~/Library --json

# Dry run by default
tidymac clean --category developer
tidymac clean --category developer --dry-run

# Print paths, then ask for confirmation
tidymac clean --category developer --execute

# Skip only the prompt
tidymac clean --category developer --execute --yes
```

`--yes` is valid only with `--execute`. It never skips path output or validation.

## Planned tabs

- Files: filter scans by size or modification time.
- Apps: review an application and related files before uninstalling it.
- Startup: inspect login items and launch services.
- SSH: show public SSH information and permission problems.
- Settings: apply reviewed, reversible system settings.
