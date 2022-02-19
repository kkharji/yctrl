# YCtrl

Thin wrapper around [yabai] that adds convenient and intuitive fixes.

By thin wrapper we mean that you could just replace `yabai` with
`yctrl` and expect everting to work as is. Though, you need to get
ride of `-m` and `--` before actions/commands. e.g.

Same as `yabai -m` cli, it communicate with yabai socket directly.

```bash
yabai -m window --focus next
yctrl window focus next
```

## Enhancements

- `inc` enhance resize that just accept either `right` or `left`. (works as you would expect)
- `next/prev`: cycles with spaces and windows on all commands.
- `focus next/prev`: accounts for floating windows (isn't that the default?).
- `focus next/prev`: If current space has only one window then window next would go to next/prev space window

## Installation

### Cargo

```bash
cargo install --git https://github.com/tami5/yctrl
```

### Flakes

```nix
{
  inputs = {
    yctrl.url = "github:tami5/yctrl";
    yctrl.inputs.nixpkgs.follows = "nixpkgs";
  };
  output = { self, ... }@inputs {
    /// ......
    {
      nixpkgs.overlays = [ inputs.yctrl.overlay ];
    };
  };
}
```

### Legacy

```nix
{
  nixpkgs.overlays = [
    (import (builtins.fetchTarball {
      url = https://github.com/tami5/yctrl/archive/master.tar.gz;
    }))
  ];
}
```


## Inspirations

- [slam/yabaictl](https://github.com/slam/yabaictl)
- [thenoim/yabai-extended-cli](https://github.com/TheNoim/yabai-extended-cli/tree/main/YabiExtendedCli)

[yabai]: https://github.com/koekeishiya/yabai
