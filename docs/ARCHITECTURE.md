# Architecture and roadmap

tidymac is a Cargo workspace with a reusable core library and a terminal application.

```text
tidymac/
├── Cargo.toml
├── crates/
│   ├── tidymac-core/
│   │   ├── src/
│   │   │   ├── scan.rs
│   │   │   ├── size.rs
│   │   │   ├── rules.rs
│   │   │   ├── safety.rs
│   │   │   └── clean.rs
│   │   └── tests/
│   └── tidymac/
│       └── src/
│           └── ui/
├── rules/
└── .github/workflows/
```

`tidymac-core` owns scanning, size calculation, rules, validation, and cleanup planning. It must not
depend on terminal rendering. The `tidymac` crate owns commands, terminal state, rendering, and
input.

## Data flow

```text
scan request
    -> directory walker
    -> file metadata
    -> indexed tree
    -> terminal or JSON output

cleanup selection
    -> rule expansion
    -> path validation
    -> path review
    -> confirmation
    -> execution-time validation
    -> Trash
```

Only the validated path type described in [Safety and cleanup rules](SAFETY.md) can reach the
cleanup executor.

## Scanner design

- Start in the current user's home directory unless another path is supplied.
- Stay on the starting device and report skipped mount points.
- Record symbolic links without following them.
- Calculate allocated size from `st_blocks * 512`.
- Deduplicate hard links by `(st_dev, st_ino)`.
- Store the tree in an indexed arena such as `Vec<Node>`.
- Stream entries, progress, and path-specific errors through a channel.
- Mark results incomplete when a folder cannot be read.

Allocated size is an estimate. Sparse files may have a larger apparent size, and APFS clones may
share extents that file metadata cannot measure. The disk gauge must use volume statistics.

## Cleanup design

Most cleanup targets are TOML rules compiled into the binary. User rules load from
`~/.config/tidymac/rules/`. Targets needing application-specific inspection implement a `Cleaner`
trait. All paths use the same compiled safety policy.

## Dependencies

Keep versions in `Cargo.toml`. Confirm each dependency with a small implementation test.

| Need | Candidate |
|---|---|
| Terminal | `ratatui` with Crossterm |
| Traversal | `ignore` and `rayon` |
| Commands | `clap` |
| TOML | `serde` and `toml` |
| Embedded rules | `include_dir` |
| Process checks | `sysinfo` or a macOS API |
| Trash | A library or macOS API that supports Finder restoration |
| Errors | `thiserror`, with `anyhow` at the application boundary if needed |

The minimum platform is macOS 13 on Apple silicon and Intel. Do not assume every path uses APFS.
Unsupported filesystem behavior must fail clearly.

## Roadmap

| Milestone | Work | Acceptance |
|---|---|---|
| M0 Foundation | Workspace, licenses, formatting, linting, and CI | Both crates build for supported targets and tests pass |
| M1 Scanner | Walk one device, report allocated blocks, deduplicate hard links, stream progress and errors | Synthetic tests cover links, sparse files, nested folders, permissions, and device boundaries |
| M2 Disk UI | Column browser, sorting, navigation, progress, warnings, and disk gauge | The interface stays responsive and marks incomplete scans |
| M3 Rules | TOML schema, loaders, reviewed rule packs, and text or JSON dry runs | Every bundled rule loads and resolves only permitted targets without changing files |
| M4 Cleanup | Validation gate, process checks, review, confirmation, and Trash support | Adversarial safety tests pass and Finder can restore a test item |
| M5 P1 release | Universal binaries, Homebrew and Cargo publishing, and Full Disk Access guidance | Automated tests and manual release checks pass on Apple silicon and Intel |
| M6 P1.5 | Large or old file filters and application uninstall review | New features use existing scan and safety systems without expanding protected paths |
| M7 P2 | Startup items, SSH dashboard, and reversible settings | Every change records its prior state and can be restored in tests |

## P1 release checks

1. Compare a known scan with macOS allocation reporting.
2. Confirm missing Full Disk Access produces clear skipped-folder warnings.
3. Review every path produced by bundled cleanup rules.
4. Move a low-risk target to the Trash and restore it with Finder.
5. Check allocation and APFS limitation labels.
6. Confirm execution prompts unless both `--execute` and `--yes` are present.
