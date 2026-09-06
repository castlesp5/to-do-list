# To-Do App

made by : ismael boujdad (version 1.0)

<img src="https://images2.memedroid.com/images/UPLOADED20/50ea417c18554.jpeg" alt="Alt text" width="300" />

A fast, keyboard-driven to-do list manager for the terminal, built in Rust with [ratatui](https://ratatui.rs/) and [crossterm](https://github.com/crossterm-rs/crossterm). Manage tasks, reorder them, mark them done, and persist your list to JSON — all without leaving the keyboard.

## Features

- **Add, edit, and delete** to-do items on the fly
- **Reorder items** up or down in the list
- **Mark items done/undone** with a single keystroke
- **Save to** and **load from** JSON files
- Clean, distraction-free terminal UI with a live status bar showing available keybindings
- Zero mouse required — fully navigable via keyboard

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain, 2021 edition or later)
- A terminal emulator that supports raw mode (most modern terminals do)

## Installation

Clone the repository and build with Cargo:

```bash
git clone git@github.com:castlesp5/to-do-list.git
cd to-do-list
cargo build --release
```

Run the app:

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./to-do-list
```

## Dependencies

| Crate         | Purpose                              |
|---------------|---------------------------------------|
| `ratatui`     | Terminal UI rendering                 |
| `crossterm`   | Terminal input/output backend         |
| `serde`       | Serialization framework               |
| `serde_json`  | JSON encoding/decoding for save/load  |

## Usage & Keybindings

The app has three modes: **NORMAL**, **INSERT**, and **EDIT**. The current mode and its available actions are always shown in the status bar at the bottom of the screen.

### Normal Mode

| Key       | Action                                  |
|-----------|------------------------------------------|
| `j`       | Move selection down                     |
| `k`       | Move selection up                       |
| `SHIFT + j`| Move selected item down in the list     |
| `SHIFT + k`| Move selected item up in the list       |
| `Enter`   | Toggle done/undone for selected item    |
| `a`       | Enter INSERT mode to add a new item     |
| `e`       | Enter EDIT mode to modify selected item |
| `d`       | Delete the selected item                |
| `s`       | Save the list to a JSON file            |
| `o`       | Load a list from a JSON file            |
| `q`       | Quit the application                    |

### Insert Mode (`a`)

| Key       | Action                          |
|-----------|-----------------------------------|
| Type      | Enter text for the new item      |
| `Enter`   | Confirm and add the new item     |
| `Esc`     | Cancel and return to NORMAL mode |

### Edit Mode (`e`)

| Key       | Action                              |
|-----------|---------------------------------------|
| Type      | Modify the item's text                |
| `Enter`   | Confirm changes                       |
| `Esc`     | Cancel and return to NORMAL mode      |

### Save / Load Prompts (`s` / `o`)

| Key       | Action                              |
|-----------|---------------------------------------|
| Type      | Enter a file path                     |
| `Enter`   | Save to (or load from) that path      |
| `Esc`     | Cancel and return to NORMAL mode      |

## Data Format

Lists are saved as pretty-printed JSON. Each item follows this schema:

```json
[
  {
    "todo": "Buy groceries",
    "done": false
  },
  {
    "todo": "Write project README",
    "done": true
  }
]
```

You can freely edit these files by hand or share them between machines, since the format is plain, human-readable JSON.

## Roadmap

Some ideas for future versions:

- Due dates and priority levels
- Persistent config for a default save path
- Filtering/search within the list
- Color themes

## License

This project is open source. Made fully by ismael boujdad as a free project
it's still in first version and it has a lot of bugs, please report the bugs as soon as possible :)

## Author

Made by [castlesp5](https://github.com/castlesp5)
