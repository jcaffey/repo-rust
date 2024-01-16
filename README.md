# repo

## usage

specify -c or --config for config file path otherwise default config is used

```
repo o|open proj|set
repo s|status proj|set
repo pull proj|set
repo push proj|set
```
## config
.config/repo/.repo.json:
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
- [ ] update help to include aliases for open and status
- [ ] show help on uknown argument
- [x] execute editor command with args
- [ ] update examples
- [ ] explain sets, dir, path, and aliases
- [ ] implement pull
- [ ] cleanup status output
