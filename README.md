# repo

## usage

specify -c or --config for config file path
default is $XDG_HOME/repo/config.json

```
repo o|open proj|set
repo s|status proj|set
repo pull proj|set
repo push proj|set

```
## config
.config/repo/config.json:
```json
{
  "repo": {
    "aliases": {
        "dotfiles": "/some/path/to/dotfiles"
    },
    "editor": {
      "command": "nvim",
      "args": ["-c", ":cd {{target}}"]
    },
    "sets": {
      "home": ["/Users/username/dotfiles", "/Users/username/repos/notes"]
    },
    "settings": {
        "root": "path/to/my-repos"
    }
  }
}
```

## examples
```sh
cargo run -- s home
```

## todo
- [ ] prettyify output for operations
- [ ] document use of {{target}} in editor args
- [ ] add list command
- [ ] split up project
- [ ] update help to include aliases for open and status
- [ ] show help on uknown argument
- [x] execute editor command with args
- [ ] update examples
- [ ] explain sets, dir, path, and aliases
- [ ] implement pull
- [ ] cleanup status output
