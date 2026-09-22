# Safety and cleanup rules

These rules apply to bundled rules, user rules, Disk tab selections, and custom cleaners.

## Cleanup contract

Every cleanup follows the same sequence:

1. Build a dry-run plan.
2. Show every resolved path.
3. Show the item count, estimated size, and Trash destination.
4. Ask for confirmation.
5. Validate each path again.
6. Move approved items to the Trash.

P1 cannot permanently delete files or empty the Trash. `--execute` enables changes and asks for
confirmation. Automation requires both `--execute` and `--yes`. The `--yes` flag skips only the
prompt, not path output or validation.

Space may remain in use until the user empties the Trash. Report this after a successful cleanup.

## Validation boundary

Rust enforces safety. TOML files cannot change it. Every filesystem target must pass through:

```rust
fn validate_deletable(path: &Path) -> Result<ValidatedPath, SafetyError>
```

`ValidatedPath` fields are private. Cleanup code cannot create one without validation.

A target must be a strict descendant of a compiled cleanup root. P1 allows narrow roots for:

- Developer caches under `~/Library/Developer`
- Named caches under `~/Library/Caches`
- Logs under `~/Library/Logs`
- Saved state under `~/Library/Saved Application State`
- Reviewed package-manager caches
- Specific Mail download caches
- Specific system logs handled by a reviewed cleaner

A rule cannot add a root or target the root itself.

## Protected paths

Always reject these protected targets:

| Path | Reason |
|---|---|
| `/System`, `/usr`, and their descendants | System files |
| `/Library` itself | Shared application and service data |
| The user's home directory | Too broad |
| `~/Documents`, `~/Desktop`, and `~/Downloads` | User files |
| `~/Library/Mobile Documents` and `~/Library/CloudStorage` | Cloud data |
| `~/Library/Keychains` and `~/.ssh` | Credentials |
| Any `.git` directory | Repository data |
| Any mounted volume root | Too broad |

Protected folders include all descendants.

Named paths under `/Library` are not P1 targets. Adding one later requires a safety review and a
compiled allow-list change.

## Path checks

Validation must:

1. Reject relative, empty, unsupported, and volume-root paths.
2. Resolve the current user's home directory.
3. Resolve existing components without trusting lexical `..` cleanup.
4. Reject symbolic links that leave an approved root or enter a protected path.
5. Confirm the target is below its compiled root.
6. Record its canonical path, device, inode, and relevant metadata.

Immediately before execution, resolve the target again and compare its path, device, and inode.
Cancel the item if any value changed. Use macOS operations that avoid following replaced symbolic
links where possible, and fail closed when identity cannot be confirmed. Document any remaining
platform limitation before M4 is complete.

## Cleanup rules

Bundled TOML rules live in `rules/`. User rules live in `~/.config/tidymac/rules/`. A user rule may
replace a bundled rule with the same `id`, but it still uses the compiled safety policy.

```toml
[[rule]]
id = "xcode-derived-data"
name = "Xcode DerivedData"
category = "developer"
tier = "safe"
paths = ["~/Library/Developer/Xcode/DerivedData/*"]
description = "Build intermediates. Xcode recreates them during the next build."
regenerates = true
requires_quit = ["com.apple.dt.Xcode"]
min_age_days = 0
```

Required fields are `id`, `name`, `category`, `tier`, `paths`, and `description`. Optional fields
are `regenerates`, `requires_quit`, and `min_age_days`.

- IDs contain lowercase letters, numbers, and hyphens.
- Categories are `developer`, `package`, `application`, `browser`, `logs`, or `system`.
- Tiers are `safe`, `caution`, or `expert`.
- Paths must be absolute. `~` is allowed only as the first component.
- Unknown fields, unsupported globs, and unsafe static paths are rejected.
- Every expanded target is validated again. No matches is a valid result.

Descriptions must say what the files contain and what removal does. Do not call files junk or
unnecessary without explaining why they are safe to remove.

## Risk tiers and running applications

| Tier | Selection behavior |
|---|---|
| `safe` | Selected by default because the data is recreated at only a time cost |
| `caution` | Selected manually because removal may require downloads, indexing, or sign-in |
| `expert` | Selected manually with typed confirmation because data may be unique |

Tiers never weaken validation or confirmation.

Rules may list application bundle identifiers that must be closed. Check them when creating the
plan and again before execution. Cancel affected targets if an application starts between checks.

Custom cleaners, such as Docker or simulator cleanup, may use an application API. They must produce
a reviewable plan and follow equivalent validation, confirmation, and process checks.

Before merging a rule, run parsing and safety tests, inspect every dry-run target, and record the
tested macOS and application versions.

## Tests

Use temporary directories to cover:

- `..` traversal, redundant separators, and paths outside approved roots
- Targets equal to a cleanup root, home directory, or volume root
- Protected paths reached through alternate spellings
- Symbolic-link escapes and links swapped in after review
- Device or inode changes after review
- Invalid UTF-8 or unsupported names
- User rules that try to broaden a bundled rule
- `--yes` without `--execute`

Automated tests must not use a real home directory. The M4 manual test must move a low-risk
temporary target to the Trash and restore it with Finder's Put Back action.
