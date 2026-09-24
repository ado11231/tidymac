# Features

* What tidymac will do, how its screens work, and what it will never do.
* Nothing here is built yet. For progress, read [ROADMAP.md](ROADMAP.md).
* For the rules tidymac follows when it changes things, read [SAFETY.md](SAFETY.md).

## Contents

1. [Before You Begin](#before-you-begin)
2. [Phases](#phases)
3. [Disk Visibility](#disk-visibility)
4. [Cleanup](#cleanup)
5. [Later Features](#later-features)
6. [Shared Features](#shared-features)
7. [Never Included](#never-included)
8. [The Terminal Interface](#the-terminal-interface)
9. [The Disk Tab](#the-disk-tab)
10. [The Clean Tab](#the-clean-tab)
11. [The SSH Tab](#the-ssh-tab)
12. [The Dotfiles Tab](#the-dotfiles-tab)
13. [The Startup Tab](#the-startup-tab)
14. [The Settings Tab](#the-settings-tab)

## Before You Begin

| Term | Meaning |
| --- | --- |
| **Size on disk** | The space a file really uses on disk. It can differ from the file's length. |
| **Hard link** | One file with more than one name. tidymac counts it once. |
| **APFS** | The file system macOS uses. On APFS, copied files can share space. |
| **Dry run** | A cleanup that lists what it would do and changes nothing. |
| **Rule** | A short TOML entry that says which files tidymac may clean, and how risky that is. |
| **Risk tier** | How risky a rule is: `safe`, `caution`, or `expert`. |
| **Dotfile** | A settings file in your home folder, such as `~/.zshrc`. |
| **ssh agent** | A background program that holds unlocked ssh keys for you. |
| **Background item** | A program macOS starts without you opening it, such as a login item, launch agent, or launch daemon. |
| **Sleep assertion** | A request from an app to keep the Mac awake. |
| **`sudo`** | Runs one command with admin rights, after asking for your password. |

## Phases

| Phase | Meaning |
| --- | --- |
| P1 | The first release. |
| P1.5 | Planned after P1. |
| P2 | Planned, but not fully designed. |

## Disk Visibility

| Phase | Feature |
| --- | --- |
| P1 | Scan the home folder. |
| P1 | Show each folder's size on disk, count hard links once, and draw a bar for each folder's share. |
| P1 | Browse folders in columns, sorted by size, name, or file count. |
| P1 | Show how full the disk is, which folders it could not read, and progress while it scans. |
| P2 | An optional treemap. |

* Folder sizes are estimates. Copied files on APFS can share space, so tidymac cannot say exactly how much a cleanup will free.

## Cleanup

| Phase | Feature |
| --- | --- |
| P1 | A dry run first. Then every path, a question, and a move to the Trash. |
| P1 | Three risk tiers: `safe`, `caution`, and `expert`. |
| P1 | Skips rules while their app is open, can skip recent files, and totals what you selected. |
| P1 | Bundled TOML rules, plus your own rules. |
| P1 | Rules for Xcode, simulators, package managers, apps, browsers, logs, saved app state, Mail, and Docker. |

* **P1 never deletes files for good and never empties the Trash.**

## Later Features

| Phase | Area | What It Does |
| --- | --- | --- |
| P1.5 | Large and old files | Filters a scan by size or by when files were last changed, and opens the results in the Disk tab. |
| P1.5 | Apps | Reviews an app and its files before removing it. Leaves anything that may be your data unselected, warns about folders shared with other apps, and refuses while the app is open. |
| P2 | Startup | Shows login items and background items, including ones that are easy to miss, and turns them off in a way you can undo. |
| P2 | SSH | Shows ssh settings, keys, the ssh agent, and known hosts. Fixes permissions, adds or removes agent keys, and removes old known hosts. |
| P2 | Dotfiles | Lists your dotfiles, edits them with a backup and a check for mistakes, and exports them without secrets. |
| P2 | Settings | Changes only settings that affect speed, battery life, or sleep. Saves the old values, so every change can be undone. |
| P2 | Saved preferences | Remembers your tidymac choices between runs. |

* File age always means when a file was last changed, not when it was last opened.
* If tidymac cannot read the login items, the Startup tab says its list may be incomplete.

## Shared Features

* Works fully from the keyboard, with help on every screen.
* One program, with nothing else to install.
* Installs through Homebrew or Cargo.
* Runs on macOS 13 or later, on Apple silicon and Intel.

## Never Included

| Request | Reason |
| --- | --- |
| Turning off System Integrity Protection | It protects macOS itself. |
| Removing Apple's own apps | macOS needs them. |
| Changing `nvram` | A mistake can stop the Mac from starting. |
| Deleting duplicate files automatically | Only you know which copy matters. |
| Freeing memory | macOS already manages memory well. |
| Cleaning with one key and no review | Every cleanup shows its paths first. |
| Deleting files for good in P1 | Everything goes to the Trash, so it can be put back. |
| Showing private ssh keys | Only key details are shown, never the key itself. |
| Creating or deleting ssh keys | Use `ssh-keygen` for that. |
| Settings that do not affect speed, battery life, or sleep | The Settings tab only covers optimization. |
| Importing or syncing dotfiles | Unpack the export into a Git repository, or use a dotfile manager such as chezmoi. |

## The Terminal Interface

* Run `tidymac` to open it.
* The only options are `--version`, which prints the version, and `--help`, which prints a short usage note. There are no other commands or options.
* The Dashboard opens first. It shows:
  1. Free and used space on the disk.
  2. Whether the last scan was complete.
  3. Cleanup categories, with links to the Clean tab.
* The Dashboard shows folder sizes and disk totals separately. They can differ because of APFS copies, snapshots, and space macOS frees on its own.

| Key | Action |
| --- | --- |
| `Tab` / `Shift-Tab` | Next or previous tab. |
| `?` | Help for the current screen. |
| `q` | Quit, when no dialog is open. |

* Arrow keys work everywhere. The Vim keys `h`, `j`, `k`, and `l` work outside text fields.

## The Disk Tab

* A column browser that starts at your home folder.

| Key | Action |
| --- | --- |
| `Right` or `Enter` | Open the selected folder. |
| `Left` | Go back to the parent folder. |
| `Up` / `Down` | Move the selection. |
| `s` | Change the sort order. |
| `d` | Plan a cleanup of the selected item. |
| `g` / `G` | Jump to the first or last row. |

* A cleanup started here goes through the same checks, review, and question as any other.

## The Clean Tab

* Each row shows a rule's name, risk tier, number of items, and estimated size.

| Key | Action |
| --- | --- |
| `Space` | Select or clear a rule. |
| `Enter` | Review the paths the selected rules found. |
| `?` | Explain the selected rule and its tier. |

| Tier | How It Is Selected |
| --- | --- |
| `safe` | Selected from the start. |
| `caution` | You select it yourself. |
| `expert` | You select it and type a confirmation. |

* A rule cannot be selected while its app is open.
* Every cleanup follows the same steps:
  1. Select rules.
  2. Review every path.
  3. Confirm the number of items, their size, and that they go to the Trash.
  4. tidymac checks each path again, then moves it to the Trash.
* The path review cannot be skipped.

## The SSH Tab

* Shows what is in `~/.ssh` without ever showing a private key.

| Section | Shows |
| --- | --- |
| Config | Each host in `~/.ssh/config`, with its host name, user, port, and key. |
| Keys | Each key's type, size, fingerprint, comment, whether it has a passphrase, and whether its `.pub` file exists. |
| Agent | The keys loaded in the ssh agent, from `ssh-add -l`. |
| Known hosts | Each entry in `~/.ssh/known_hosts`. |
| Permissions | Files and folders that other users can read or change. |

* It can make three changes:

| Action | What Happens |
| --- | --- |
| Fix permissions | Sets `~/.ssh` to `700`, private keys and `config` to `600`, and public keys to `644`. The old permissions are saved so you can undo. |
| Agent keys | Adds a key with `ssh-add`, or removes it with `ssh-add -d`. Key files are not touched. |
| Old hosts | Removes the entries you select with `ssh-keygen -R`, which keeps `known_hosts.old`. tidymac also saves its own backup. |

* Each change shows what it will do and asks first.
* To edit `~/.ssh/config`, use the Dotfiles tab.

## The Dotfiles Tab

* Lists the dotfiles tidymac knows about, but only the ones on your Mac. If you have no tmux settings, there is no tmux row.

| Group | Files |
| --- | --- |
| Shell | `~/.zshrc`, `~/.zprofile`, `~/.zshenv`, `~/.bashrc`, `~/.bash_profile`, `~/.profile`, `~/.config/fish/config.fish` |
| Git | `~/.gitconfig`, `~/.config/git/config`, `~/.config/git/ignore`, `~/.gitignore_global` |
| SSH | `~/.ssh/config` |
| Editors | `~/.vimrc`, `~/.config/nvim/init.lua`, `~/.config/nvim/init.vim` |
| Terminal | `~/.tmux.conf`, `~/.config/tmux/tmux.conf`, `~/.config/starship.toml`, `~/.inputrc` |

### Edit A Dotfile

1. tidymac saves a backup in `~/.local/state/tidymac/backups/`, named with the date and time.
2. It opens the file in your editor, from `$VISUAL` or `$EDITOR`.
3. When you close the editor, it shows what changed.
4. It checks the file for mistakes, where a check exists:

   | File | Check |
   | --- | --- |
   | zsh | `zsh -n` |
   | bash | `bash -n` |
   | Git | `git config --list --file` |
   | SSH | `ssh -G` |

5. You keep the change, edit again, or restore the backup.

### Export Dotfiles

1. Select the files to export.
2. tidymac looks for anything that looks like a token or password, and shows you each match.
3. Choose where to save it. The default is your home folder.
4. It writes a `.tar.gz` archive, with a list of what was included and what was left out.

* These are never exported: private keys, `~/.netrc`, `~/.aws/credentials`, `~/.npmrc`, and `~/.pypirc`.
* To keep dotfiles in Git, unpack the archive into a repository.

## The Startup Tab

* Shows every program macOS starts without you opening it. Many never appear in the Dock or menu bar.

| Section | Shows | Read From |
| --- | --- | --- |
| Login items | Apps that open when you log in. | The login item list |
| Background items | Items listed under Allow in the Background in System Settings. | `sfltool dumpbtm` |
| Launch agents | Programs that run while you are logged in. | `~/Library/LaunchAgents`, `/Library/LaunchAgents` |
| Launch daemons | Programs that run for the whole Mac, even before anyone logs in. | `/Library/LaunchDaemons` |

* Each row shows the program, who made it, whether it is running, and whether it is signed. Signed means macOS can confirm who made it.
* Items under `/System/Library` are shown, but can never be changed.
* Turning an item off:
  1. tidymac shows the exact command, such as `launchctl disable gui/501/com.example.helper`.
  2. It saves the item's current state.
  3. It runs the command, with `sudo` when the item runs for the whole Mac.
* Turning it back on puts back the saved state.
* tidymac never edits or deletes a launch agent or launch daemon file.
* If tidymac cannot read the background items, the tab says its list may be incomplete.

## The Settings Tab

* Shows only settings that change speed, battery life, or sleep.
* Sections that do not apply to this Mac are hidden.

### Power Mode

| Preset | Effect |
| --- | --- |
| Battery Life | Turns on Low Power Mode. Slower and cooler, and the battery lasts longer. |
| Balanced | The macOS default. |
| Maximum Performance | Turns on High Power Mode, on Macs that support it. Faster during long, heavy work, with more fan noise and power use. |

* Set separately for battery and for the charger.
* Uses `pmset`. The exact setting names differ between macOS versions, and must be checked on a real Mac before this is built.

### Graphics Switching

* Only on Intel Macs with two GPUs.
* Turns Automatic Graphics Switching on or off, with `pmset gpuswitch`.
* On saves battery by using the built in GPU for light work. Off always uses the faster GPU.

### Refresh Rate

* Lists each display, including ProMotion and supported external displays, with:
  1. Its current refresh rate.
  2. The rates it offers.
  3. Whether it supports Adaptive Sync.
* Switching:
  1. Pick a rate. Higher is smoother. Lower uses less power.
  2. tidymac switches the display, then asks whether to keep it.
  3. If you do not answer within 15 seconds, it switches back. So if the screen goes blank, it fixes itself.
* Uses CoreGraphics display modes.

### Game Mode

* Shows whether this Mac offers Game Mode.
* Explains how to get it: open a supported game in full screen. macOS then turns Game Mode on and lets the game use the CPU and GPU first.
* tidymac cannot turn Game Mode on. It only explains.

### What Keeps Your Mac Awake

| View | Shows | Read From |
| --- | --- | --- |
| Now | Apps keeping the Mac awake right now, and the reason each gives. | `pmset -g assertions` |
| Overnight | Each time the Mac woke while asleep, and what woke it. | `pmset -g log` |

* Each row links to the app or setting that caused it. It can show the app in Finder, or jump to the matching wake setting below.
* This view changes nothing. tidymac never quits an app.

### Wake Settings

| Setting | Turning It Off Means | `pmset` Key |
| --- | --- | --- |
| Wake for network access | The Mac no longer wakes for network requests, such as file sharing. | `womp` |
| Power Nap | The Mac no longer checks mail or runs backups while asleep. | `powernap` |
| Keep network connections alive | Find My and notifications stop while the Mac sleeps. | `tcpkeepalive` |

### Every Change

1. tidymac shows the exact command it will run.
2. It saves the current value.
3. It runs the command with `sudo`, which asks for your password.
4. Undo puts back the saved value.
