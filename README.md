# TUIMENU - A simple program launcher 

### Adding Programs

To add a program to tuimenu, navigate to `$Home/tuimenu/list.json` which is set up like below

```json
[
  {
    "cmd": "{NAME OF COMMAND}",
    "desc": "{TEXT TO BE SHOWN WITH COMMAND}",
    "args": ["ARG", "BY", "SPACE"] //Optional
  },
```

### Example
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
