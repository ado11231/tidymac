# Safety

* The rules tidymac follows whenever it changes a file or a setting.
* They cover every tab that changes something: Disk, Clean, SSH, Dotfiles, Startup, and Settings. They also cover bundled rules, your own rules, and custom cleaners.
* Nothing here is built yet. For progress, read [ROADMAP.md](ROADMAP.md).

## Contents

1. [Key Terms](#key-terms)
2. [How A Cleanup Runs](#how-a-cleanup-runs)
3. [The Path Check](#the-path-check)
4. [Protected Paths](#protected-paths)
5. [What The Path Check Does](#what-the-path-check-does)
6. [Cleanup Rules](#cleanup-rules)
7. [Risk Tiers And Running Apps](#risk-tiers-and-running-apps)
8. [SSH And Dotfile Changes](#ssh-and-dotfile-changes)
9. [Admin Rights](#admin-rights)
10. [Settings And Startup Changes](#settings-and-startup-changes)
11. [Tests](#tests)

## Key Terms

| Term | Meaning |
| --- | --- |
| **Cleanup root** | A folder that cleanup may remove items from. The list is fixed in the Rust code. |
| **Protected path** | A folder tidymac never touches, including everything inside it. |
| **Symbolic link** | A file that points to another path. |
| **Real path** | A path after following every symbolic link and `..`. |
| **Device and inode** | Two numbers that identify a file, even if it is renamed. |
| **Allow list** | A fixed list, in the Rust code, of the only files or settings a feature may change. |
| **`sudo`** | Runs one command with admin rights, after asking for your password. |

## How A Cleanup Runs

1. Make a plan without changing anything. This is a dry run.
2. Show every path the plan found.
3. Show how many items there are, how much space they use, and that they go to the Trash.
4. Ask you to confirm.
5. Check each path again.
6. Move the items to the Trash.

* P1 never deletes files for good and never empties the Trash.
* Cleanup only happens in the terminal interface. There is no command or option that skips the path list or the question.
* After a cleanup, tidymac reminds you the space is only freed once you empty the Trash.

## The Path Check

* Safety lives in the Rust code. TOML rules cannot change it.
* Every path must pass one function before it can be moved:

  ```rust
  fn validate_deletable(path: &Path) -> Result<ValidatedPath, SafetyError>
  ```

* Only this function can make a `ValidatedPath`, so no other code can skip the check.
* A path must be inside a cleanup root, and never the root itself.
* P1 has these cleanup roots:
  1. Developer caches in `~/Library/Developer`.
  2. Named caches in `~/Library/Caches`.
  3. Logs in `~/Library/Logs`.
  4. Saved app state in `~/Library/Saved Application State`.
  5. Package manager caches that have been reviewed.
  6. Certain Mail download caches.
  7. Certain system logs, cleaned by a reviewed cleaner.
* A rule cannot add a cleanup root.

## Protected Paths

* tidymac always refuses these, and everything inside them:

| Path | Why |
| --- | --- |
| `/System`, `/usr` | macOS system files. |
| `/Library` itself | Shared by apps and services. |
| Your home folder itself | Far too broad. |
| `~/Documents`, `~/Desktop`, `~/Downloads` | Your own files. |
| `~/Library/Mobile Documents`, `~/Library/CloudStorage` | iCloud and other cloud files. |
| `~/Library/Keychains`, `~/.ssh` | Passwords and keys. |
| Any `.git` folder | Repository history. |
| The top folder of any disk | Far too broad. |

* Folders inside `/Library` are not P1 targets. Adding one later needs a safety review and a change to the Rust code.

## What The Path Check Does

1. Refuses empty paths, relative paths, paths it does not support, and the top folder of any disk.
2. Finds your home folder.
3. Follows each part of the path on disk. It does not just remove `..` from the text.
4. Refuses symbolic links that lead out of a cleanup root or into a protected path.
5. Confirms the path is inside its cleanup root.
6. Saves the real path, device, inode, and other details it needs.

* Right before moving an item, tidymac checks its real path, device, and inode again. If anything changed, it skips that item.
* Where macOS allows it, tidymac moves files in a way that cannot be tricked by a link swapped in after the review.
* If tidymac cannot be sure a file is the same one you reviewed, it skips it.
* Any case macOS cannot protect against must be written down before M4 is done.

## Cleanup Rules

* Bundled rules live in `rules/`. Your own rules live in `~/.config/tidymac/rules/`.
* Your rule can replace a bundled rule by using the same `id`. It still goes through the same Rust checks.

```toml
[[rule]]
id = "xcode-derived-data"
name = "Xcode DerivedData"
category = "developer"
tier = "safe"
paths = ["~/Library/Developer/Xcode/DerivedData/*"]
description = "Build files. Xcode makes them again during the next build."
regenerates = true
requires_quit = ["com.apple.dt.Xcode"]
min_age_days = 0
```

| Field | Required | Meaning |
| --- | --- | --- |
| `id` | Yes | Lowercase letters, numbers, and hyphens. |
| `name` | Yes | The name shown in the Clean tab. |
| `category` | Yes | `developer`, `package`, `application`, `browser`, `logs`, or `system`. |
| `tier` | Yes | `safe`, `caution`, or `expert`. |
| `paths` | Yes | Full paths. `~` may only be used at the start. |
| `description` | Yes | What the files are, and what happens when they are removed. |
| `regenerates` | No | Whether the app makes the files again. |
| `requires_quit` | No | The IDs of apps that must be closed first. |
| `min_age_days` | No | Skip files changed within this many days. |

* tidymac refuses unknown fields, patterns it does not support, and fixed paths that are not safe.
* Every path a pattern matches is checked again. Matching nothing is fine.
* Never call files junk or useless without saying why they are safe to remove.
* Before adding a rule:
  1. Run the rule and safety tests.
  2. Read every path its dry run finds.
  3. Write down the macOS and app versions you tested with.

## Risk Tiers And Running Apps

| Tier | How It Is Selected | Why |
| --- | --- | --- |
| `safe` | Already selected. | The files come back, and the only cost is time. |
| `caution` | You select it. | You may need to download, index, or sign in again. |
| `expert` | You select it and type a confirmation. | The files may not exist anywhere else. |

* A tier never skips a check or the question.
* tidymac checks the apps in `requires_quit` when it makes the plan, and again before moving files. If one of them opened in between, its items are skipped.
* Custom cleaners, such as Docker or simulator cleanup, may use the app's own tool. They still show a plan, and go through the same checks and question.

## SSH And Dotfile Changes

* These change files where they are, so they do not use `ValidatedPath` or the Trash. They have their own allow list instead.

| Feature | May Change |
| --- | --- |
| Dotfile editing | Only the files listed in the Dotfiles tab. |
| SSH permissions | `~/.ssh` and the files directly inside it. |
| Known hosts | `~/.ssh/known_hosts`, only through `ssh-keygen -R`. |
| Agent keys | Nothing on disk. `ssh-add` only changes the running agent. |

* Every change must:
  1. Save the old content or permissions first, so it can be undone.
  2. Refuse symbolic links that lead out of your home folder.
  3. Write to a temporary file in the same folder, then swap it in, so a crash never leaves half a file.
* Cleanup can still never touch `~/.ssh`.
* tidymac never reads what is inside a private key. Key details come from `ssh-keygen`.
* Exports:
  1. Never include private keys, `~/.netrc`, `~/.aws/credentials`, `~/.npmrc`, or `~/.pypirc`.
  2. Show you anything that looks like a token or password before writing the archive.

## Admin Rights

* tidymac never runs as root. If you start it with `sudo`, it stops and explains why: it would find root's home folder instead of yours.
* A change that needs admin rights runs one command with `sudo`, after showing you that command.
* The terminal screen pauses while `sudo` asks for your password, then comes back.
* Cleanup never uses `sudo`.

## Settings And Startup Changes

| Feature | May Change | How |
| --- | --- | --- |
| Power mode | Low Power Mode and High Power Mode, for battery and charger | `sudo pmset` |
| Graphics switching | `gpuswitch` | `sudo pmset` |
| Wake settings | `womp`, `powernap`, `tcpkeepalive` | `sudo pmset` |
| Refresh rate | Which mode a connected display uses | CoreGraphics |
| Startup items | Whether a login item, launch agent, or launch daemon runs | `launchctl` or the login item list, with `sudo` for launch daemons |

* tidymac refuses any `pmset` setting not in this table.
* Before each change, tidymac saves the old value in `~/.local/state/tidymac/`. Undo puts it back.
* A new refresh rate switches back after 15 seconds unless you keep it.
* Items in `/System/Library` are never changed.
* Launch agent and launch daemon files are never edited or deleted. tidymac only changes whether they run.

## Tests

* Use temporary folders to test:
  1. `..` tricks, doubled slashes, and paths outside cleanup roots.
  2. A path that is a cleanup root, the home folder, or the top folder of a disk.
  3. Protected paths written in other ways.
  4. Symbolic links that lead out, and links swapped in after the review.
  5. A device or inode that changed after the review.
  6. Names that are not valid `UTF-8`, or that tidymac does not support.
  7. Your own rules that try to reach more than a bundled rule.
  8. Dotfile edits outside the allow list, or through a symbolic link that leads out.
  9. Permission fixes and dotfile edits undone from their backups.
  10. Exports that contain a private key or something that looks like a token.
  11. Starting tidymac as root.
  12. `pmset` settings outside the allow list.
  13. Settings and startup items undone from their saved values.
  14. A refresh rate that is not kept switching back.
* Automated tests never use a real home folder, and never change real Mac settings.
* The M4 manual test moves a harmless temporary file to the Trash, then restores it with Finder's Put Back.
