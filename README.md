<h1 align="center">tidymac</h1>

<p align="center">See what is filling your Mac's disk, and safely clear files your apps can make again. All from one terminal program.</p>

<br>

* **Status:** in development. Only the start of the scanner is built. Nothing below works yet.

**1. Install tidymac (Planned)**

```sh
brew install tidymac
```

or:

```sh
cargo install tidymac
```

**2. Open It**

```sh
tidymac
```

**3. Use The Tabs**

| Tab | What It Does |
| --- | --- |
| Dashboard | Shows how full the disk is, and what can be cleaned. |
| Disk | Browses your folders by size. |
| Clean | Moves files your apps can make again to the Trash, after you review every path. |

* Everything happens inside the program. The only options are `--version` and `--help`.

<br>

* Needs macOS 13 or later, on Apple silicon or Intel.
* Some folders need Full Disk Access, a macOS permission. Without it, tidymac lists the folders it skipped.
* Cleanup never deletes files for good. Everything goes to the Trash.
* Cleanup never touches your own folders, such as Documents, Desktop, Downloads, iCloud Drive, Keychains, and `~/.ssh`.

<p align="center">
  <a href="docs/FEATURES.md">Features</a> ·
  <a href="docs/SAFETY.md">Safety</a> ·
  <a href="docs/ARCHITECTURE.md">Architecture</a> ·
  <a href="docs/ROADMAP.md">Roadmap</a>
</p>
