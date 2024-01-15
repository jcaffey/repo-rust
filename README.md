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
    "editor": {
      "command": "nvim",
      "args": ["-c", ":cd {{target}}"]
    },
    "sets": {
      "home": ["/Users/username/dotfiles", "/Users/username/repos/notes"]
    }
  }
}
```

## examples
```sh
cargo run -- status home
```
