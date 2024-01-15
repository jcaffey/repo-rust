# repo

## usage

specify -c or --config for config file path otherwise default config is used

```
repo o|open proj|set
repo s|status proj|set
repo pull proj|set
repo push proj|set
```

.config/repo/.repo.json:
```json
"repo": {
    "editor": {
        program: "nvim",
        arguments: "-my args"
    },
    "sets": {
        "key": ['path/to/proj', 'path/to/other']
    }
}
```
