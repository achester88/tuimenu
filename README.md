# TUIMENU - A simple tui/cli program launcher 
<img width="1226" height="636" alt="2025-09-24-171047_hyprshot" src="https://github.com/user-attachments/assets/cc7c85a7-063c-4f73-94d8-540a6bd01c38" />

tuimenu is a simple Rust-based program intended for easily opening tui and cli applications from a simple terminal. It allows for simple "aliasing" of commands and quick launching of tui programs from one menu.

### Installation

### Adding Programs
To add a program to tuimenu, navigate to `~/tuimenu/list.json`, which is written in josn, with each program having it's own object.

```json
[
  {
    "cmd": "{NAME OF COMMAND}",
    "desc": "{TEXT TO BE SHOWN WITH COMMAND}",
    "args": ["ARG", "BY", "SPACE"] //Optional
  },
```

### Example Setup

<img width="1226" height="636" alt="2025-09-24-170954_hyprshot" src="https://github.com/user-attachments/assets/eead088a-5c1c-48c6-a624-15f4fca0a973" />

<hr>
tuimenu/list.json

```json
[
  {
    "cmd": "btop",
    "desc": "A terminal monitor of resources"
  },
  {
    "cmd": "dooit",
    "desc": "A TUI todo manager for the terminal"
  },
  {
    "cmd": "wofi",
    "desc": "A graphical program launcher",
    "args": ["--show", "drun"]
  }
]
```
