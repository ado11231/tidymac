# tidymac

tidymac is a planned macOS terminal tool for viewing disk usage and safely removing files that can
be recreated. It combines a disk browser and cleanup tool in one binary.

## Project status

The Cargo workspace is ready. Application features are not implemented yet.

The `docs/` folder defines the scope, safety rules, interface, and planned implementation.

## Phase 1

P1 has two goals:

- Browse the current user's home directory and show each folder's allocated size, file count, and
  relative size.
- Review and remove supported caches, logs, and build artifacts that macOS or their applications
  can recreate.

Users can also scan a narrower path. P1 will not claim to scan every file on the startup disk.

## Safety

Cleanup follows these rules:

1. A cleanup starts as a dry run.
2. The complete resolved path list is shown before confirmation.
3. Every target passes through validation implemented in Rust.
4. Protected locations such as Documents, Desktop, Downloads, iCloud Drive, Keychains, and
   `~/.ssh` are rejected.
5. P1 moves items to the Trash. It does not permanently delete them or empty the Trash.

See [Safety model](docs/SAFETY.md) for the full requirements.

## Planned installation

```sh
brew install tidymac
```

or:

```sh
cargo install tidymac
```

The minimum planned version is macOS 13. Releases will support Apple silicon and Intel Macs.

Some folders require Full Disk Access. If access is missing, tidymac will report skipped folders.

## Documentation

| Document | Purpose |
|---|---|
| [Features and interface](docs/FEATURES.md) | Product scope, terminal behavior, and commands |
| [Safety and cleanup rules](docs/SAFETY.md) | Path controls, confirmation, and rule format |
| [Architecture and roadmap](docs/ARCHITECTURE.md) | Technical design, milestones, and release checks |

## License

This project is dual-licensed under [MIT](LICENSE-MIT) or
[Apache 2.0](LICENSE-APACHE), at your option.
